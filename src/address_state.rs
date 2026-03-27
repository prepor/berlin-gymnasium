use leptos::prelude::*;

/// A saved address with display text and coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct SavedAddress {
    pub text: String,
    pub lat: f64,
    pub lng: f64,
}

/// Provide the saved address signal via Leptos context.
/// Reads initial value from localStorage on startup.
pub fn provide_saved_address() {
    let initial = load_from_storage();
    let signal = RwSignal::new(initial);
    provide_context(signal);
}

/// Get the saved address context signal.
pub fn use_saved_address() -> RwSignal<Option<SavedAddress>> {
    expect_context::<RwSignal<Option<SavedAddress>>>()
}

/// Save an address to the signal and localStorage.
pub fn save_address(signal: RwSignal<Option<SavedAddress>>, text: String, lat: f64, lng: f64) {
    let addr = SavedAddress { text, lat, lng };
    write_to_storage(&addr);
    signal.set(Some(addr));
}

/// Clear the saved address from signal and localStorage.
pub fn clear_address(signal: RwSignal<Option<SavedAddress>>) {
    remove_from_storage();
    signal.set(None);
}

// --- localStorage helpers ---

const STORAGE_KEY: &str = "saved_address";

fn load_from_storage() -> Option<SavedAddress> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let json = storage.get_item(STORAGE_KEY).ok()??;
    // Simple JSON: {"text":"...","lat":52.52,"lng":13.405}
    parse_json(&json)
}

fn write_to_storage(addr: &SavedAddress) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let json = format!(
            r#"{{"text":"{}","lat":{},"lng":{}}}"#,
            addr.text.replace('"', r#"\""#),
            addr.lat,
            addr.lng
        );
        let _ = storage.set_item(STORAGE_KEY, &json);
    }
}

fn remove_from_storage() {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(STORAGE_KEY);
    }
}

fn parse_json(json: &str) -> Option<SavedAddress> {
    // Minimal JSON parsing without serde_json (avoid adding a dependency).
    // Expected format: {"text":"...","lat":52.52,"lng":13.405}
    let text = extract_string_field(json, "text")?;
    let lat = extract_number_field(json, "lat")?;
    let lng = extract_number_field(json, "lng")?;
    Some(SavedAddress { text, lat, lng })
}

fn extract_string_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!(r#""{}":""#, field);
    let start = json.find(&pattern)? + pattern.len();
    let rest = &json[start..];
    // Find closing quote (handle escaped quotes)
    let mut end = 0;
    let bytes = rest.as_bytes();
    while end < bytes.len() {
        if bytes[end] == b'"' && (end == 0 || bytes[end - 1] != b'\\') {
            break;
        }
        end += 1;
    }
    Some(rest[..end].replace(r#"\""#, "\""))
}

fn extract_number_field(json: &str, field: &str) -> Option<f64> {
    let pattern = format!(r#""{}":"#, field);
    let start = json.find(&pattern)? + pattern.len();
    let rest = &json[start..];
    let end = rest.find(|c: char| c != '-' && c != '.' && !c.is_ascii_digit())?;
    rest[..end].parse().ok()
}
