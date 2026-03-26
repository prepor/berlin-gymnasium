use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonResponse {
    pub features: Vec<PhotonFeature>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonFeature {
    pub geometry: PhotonGeometry,
    pub properties: PhotonProperties,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonGeometry {
    pub coordinates: Vec<f64>, // [lng, lat] -- GeoJSON order!
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonProperties {
    pub name: Option<String>,
    pub street: Option<String>,
    pub housenumber: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

impl PhotonFeature {
    /// Extract latitude from GeoJSON coordinates (which are [lng, lat]).
    pub fn lat(&self) -> f64 {
        self.geometry.coordinates[1]
    }

    /// Extract longitude from GeoJSON coordinates (which are [lng, lat]).
    pub fn lng(&self) -> f64 {
        self.geometry.coordinates[0]
    }

    /// Build a display label for the dropdown.
    /// Format: "Name, Street Housenumber, Postcode City"
    /// Omits parts that are None. Falls back to name if nothing else is available.
    pub fn display_label(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        // Include the place name if it differs from the street
        if let Some(name) = &self.properties.name {
            let dominated_by_street = self
                .properties
                .street
                .as_deref()
                .map(|s| s == name.as_str())
                .unwrap_or(false);
            if !dominated_by_street {
                parts.push(name.clone());
            }
        }

        // Street + housenumber
        match (&self.properties.street, &self.properties.housenumber) {
            (Some(st), Some(hn)) => parts.push(format!("{} {}", st, hn)),
            (Some(st), None) => parts.push(st.clone()),
            _ => {}
        }

        // Postcode + city combined
        match (&self.properties.postcode, &self.properties.city) {
            (Some(pc), Some(city)) => parts.push(format!("{} {}", pc, city)),
            (None, Some(city)) => parts.push(city.clone()),
            (Some(pc), None) => parts.push(pc.clone()),
            _ => {}
        }

        if parts.is_empty() {
            self.properties.name.clone().unwrap_or_default()
        } else {
            parts.join(", ")
        }
    }
}

/// Geocode an address string using the Photon API (photon.komoot.io).
/// Biased toward Berlin (lat=52.52, lon=13.405). Returns up to 5 suggestions.
///
/// Uses gloo-net's `.query()` builder to pass params separately from the base URL.
/// Embedding params directly in the URL string causes gloo-net 0.6 to append a
/// trailing `&` (via its RequestBuilder → Request conversion), which Photon rejects
/// with HTTP 400.
pub async fn geocode_address(query: &str) -> Result<Vec<PhotonFeature>, String> {
    let resp = gloo_net::http::Request::get("https://photon.komoot.io/api/")
        .query([
            ("q", query),
            ("lat", "52.52"),
            ("lon", "13.405"),
            ("limit", "5"),
            ("lang", "de"),
        ])
        .send()
        .await
        .map_err(|e| format!("Geocoding-Fehler: {:?}", e))?;
    if !resp.ok() {
        return Err(format!("Geocoding HTTP {}", resp.status()));
    }
    let data: PhotonResponse = resp
        .json()
        .await
        .map_err(|e| format!("Geocoding Parse-Fehler: {:?}", e))?;
    Ok(data.features)
}
