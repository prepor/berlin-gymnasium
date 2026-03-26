use std::collections::HashMap;

use serde::Deserialize;

use crate::models::TravelTimes;

#[derive(Clone, Debug, Deserialize)]
pub struct ValhallaMatrixResponse {
    pub sources_to_targets: Vec<Vec<ValhallaMatrixEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValhallaMatrixEntry {
    pub time: Option<f64>,     // seconds, None if unreachable
    pub distance: Option<f64>, // km
    pub to_index: usize,
}

/// Fetch travel times from one origin to multiple targets using the Valhalla matrix API.
/// Tries POST first; if CORS fails, falls back to GET with `?json=` query parameter.
/// Returns a Vec of Option<u32> (minutes) in the same order as `targets`.
pub async fn fetch_travel_times_matrix(
    origin_lat: f64,
    origin_lng: f64,
    targets: &[(f64, f64)], // (lat, lng) pairs
    costing: &str,           // "pedestrian", "bicycle", or "auto"
) -> Result<Vec<Option<u32>>, String> {
    let body = serde_json::json!({
        "sources": [{"lat": origin_lat, "lon": origin_lng}],
        "targets": targets.iter().map(|(lat, lng)| {
            serde_json::json!({"lat": lat, "lon": lng})
        }).collect::<Vec<_>>(),
        "costing": costing,
    });

    let body_str = body.to_string();

    // Try POST first
    let resp = gloo_net::http::Request::post(
        "https://valhalla1.openstreetmap.de/sources_to_targets",
    )
    .header("Content-Type", "application/json")
    .body(&body_str)
    .map_err(|e| format!("Request-Fehler: {:?}", e))?
    .send()
    .await;

    // If POST fails (likely CORS), try GET with json query param
    let resp = match resp {
        Ok(r) if r.ok() => r,
        _ => {
            let encoded = js_sys::encode_uri_component(&body_str);
            let url = format!(
                "https://valhalla1.openstreetmap.de/sources_to_targets?json={}",
                encoded
            );
            gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| format!("Valhalla-Fehler: {:?}", e))?
        }
    };

    if !resp.ok() {
        return Err(format!("Valhalla HTTP {}", resp.status()));
    }

    let data: ValhallaMatrixResponse = resp
        .json()
        .await
        .map_err(|e| format!("Valhalla Parse-Fehler: {:?}", e))?;

    // Extract times: first (only) source row, convert seconds -> rounded minutes (per D-16)
    let times: Vec<Option<u32>> = data
        .sources_to_targets
        .first()
        .map(|row| {
            row.iter()
                .map(|entry| entry.time.map(|t| (t / 60.0).round() as u32))
                .collect()
        })
        .unwrap_or_default();

    Ok(times)
}

/// Fetch travel times for all three modes (walk, bike, car) and build a
/// HashMap<school_id, TravelTimes>.
///
/// `school_coords` is Vec<(school_id, lat, lng)> -- maintains index mapping
/// so Valhalla response indices map back to the correct school.
pub async fn fetch_all_travel_times(
    origin_lat: f64,
    origin_lng: f64,
    school_coords: Vec<(String, f64, f64)>,
) -> Result<HashMap<String, TravelTimes>, String> {
    let targets: Vec<(f64, f64)> = school_coords
        .iter()
        .map(|(_, lat, lng)| (*lat, *lng))
        .collect();

    // Fire all three modes sequentially (WASM is single-threaded,
    // but these are I/O-bound so interleaving via await is fine)
    let walk = fetch_travel_times_matrix(origin_lat, origin_lng, &targets, "pedestrian").await;
    let bike = fetch_travel_times_matrix(origin_lat, origin_lng, &targets, "bicycle").await;
    let car = fetch_travel_times_matrix(origin_lat, origin_lng, &targets, "auto").await;

    let walk_times = walk.unwrap_or_default();
    let bike_times = bike.unwrap_or_default();
    let car_times = car.unwrap_or_default();

    let mut result = HashMap::new();
    for (i, (school_id, _, _)) in school_coords.iter().enumerate() {
        result.insert(
            school_id.clone(),
            TravelTimes {
                walk_minutes: walk_times.get(i).copied().flatten(),
                bike_minutes: bike_times.get(i).copied().flatten(),
                car_minutes: car_times.get(i).copied().flatten(),
            },
        );
    }

    Ok(result)
}
