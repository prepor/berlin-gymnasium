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
    /// Joins street, housenumber, postcode, city. Falls back to name if all are None.
    pub fn display_label(&self) -> String {
        let parts: Vec<&str> = [
            self.properties.street.as_deref(),
            self.properties.housenumber.as_deref(),
            self.properties.postcode.as_deref(),
            self.properties.city.as_deref(),
        ]
        .iter()
        .filter_map(|p| *p)
        .collect();
        if parts.is_empty() {
            self.properties.name.clone().unwrap_or_default()
        } else {
            parts.join(", ")
        }
    }
}

/// Geocode an address string using the Photon API (photon.komoot.io).
/// Biased toward Berlin (lat=52.52, lon=13.405). Returns up to 5 suggestions.
pub async fn geocode_address(query: &str) -> Result<Vec<PhotonFeature>, String> {
    let url = format!(
        "https://photon.komoot.io/api/?q={}&lat=52.52&lon=13.405&limit=5&lang=de",
        js_sys::encode_uri_component(query)
    );
    let resp = gloo_net::http::Request::get(&url)
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
