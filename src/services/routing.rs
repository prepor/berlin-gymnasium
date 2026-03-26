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

    // If POST fails (likely CORS), try GET with json query param.
    // Uses .query() builder to avoid gloo-net trailing "&" bug (see geocoding.rs).
    let resp = match resp {
        Ok(r) if r.ok() => r,
        _ => {
            gloo_net::http::Request::get(
                "https://valhalla1.openstreetmap.de/sources_to_targets",
            )
            .query([("json", body_str.as_str())])
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

    // Walk and bike handle 106 targets fine in a single request.
    // Car ("auto") causes Valhalla FOSSGIS to 504 with 106 targets,
    // so we batch it into chunks of 25.
    let walk = fetch_travel_times_matrix(origin_lat, origin_lng, &targets, "pedestrian").await;
    let bike = fetch_travel_times_matrix(origin_lat, origin_lng, &targets, "bicycle").await;

    // Batch car requests
    let mut car_times_all: Vec<Option<u32>> = Vec::with_capacity(targets.len());
    let chunk_size = 25;
    for chunk in targets.chunks(chunk_size) {
        match fetch_travel_times_matrix(origin_lat, origin_lng, chunk, "auto").await {
            Ok(times) => car_times_all.extend(times),
            Err(e) => {
                log::warn!("Valhalla auto batch failed: {e}");
                car_times_all.extend(std::iter::repeat(None).take(chunk.len()));
            }
        }
    }

    let walk_times = walk.unwrap_or_default();
    let bike_times = bike.unwrap_or_default();
    let car_times = car_times_all;

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
