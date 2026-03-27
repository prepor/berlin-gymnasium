use std::collections::HashMap;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Reactive hash-based location context.
/// URL format: `#/path?key=value&key2=value2`
#[derive(Clone, Copy)]
pub struct HashLocation {
    pub path: RwSignal<String>,
    pub query: RwSignal<HashMap<String, String>>,
}

fn current_hash() -> (String, HashMap<String, String>) {
    let hash = web_sys::window()
        .unwrap()
        .location()
        .hash()
        .unwrap_or_default();
    parse_hash(&hash)
}

fn parse_hash(hash: &str) -> (String, HashMap<String, String>) {
    let hash = hash.strip_prefix('#').unwrap_or(hash);
    let hash = if hash.is_empty() { "/" } else { hash };

    if let Some((path, query_str)) = hash.split_once('?') {
        (path.to_string(), parse_query_string(query_str))
    } else {
        (hash.to_string(), HashMap::new())
    }
}

fn parse_query_string(qs: &str) -> HashMap<String, String> {
    qs.split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            if key.is_empty() {
                None
            } else {
                let decoded = js_sys::decode_uri_component(value)
                    .map(|s| String::from(s))
                    .unwrap_or_else(|_| value.to_string());
                Some((key.to_string(), decoded))
            }
        })
        .collect()
}

/// Initialize hash-based routing and provide context.
pub fn provide_hash_router() {
    let (path, query) = current_hash();
    let location = HashLocation {
        path: RwSignal::new(path),
        query: RwSignal::new(query),
    };

    provide_context(location);

    // Listen for hashchange events (covers link clicks + back/forward)
    let update = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let (new_path, new_query) = current_hash();
        update_location_signals(&location, new_path, new_query);
    }) as Box<dyn FnMut(_)>);

    let window = web_sys::window().unwrap();
    window
        .add_event_listener_with_callback("hashchange", update.as_ref().unchecked_ref())
        .unwrap();
    // Also listen to popstate for replaceState-based navigations
    let update2 = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let (new_path, new_query) = current_hash();
        update_location_signals(&location, new_path, new_query);
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("popstate", update2.as_ref().unchecked_ref())
        .unwrap();

    update.forget();
    update2.forget();
}

/// Only set signals when their values actually change, to avoid unnecessary
/// re-renders (e.g., recreating the entire ListingPage on every filter change).
fn update_location_signals(
    location: &HashLocation,
    new_path: String,
    new_query: HashMap<String, String>,
) {
    if location.path.get_untracked() != new_path {
        location.path.set(new_path);
    }
    if location.query.get_untracked() != new_query {
        location.query.set(new_query);
    }
}

/// Navigate to a hash URL. If `replace` is true, replaces current history entry.
pub fn navigate_hash(url: &str, replace: bool) {
    let hash_url = if url.starts_with('#') {
        url.to_string()
    } else {
        format!("#{}", url)
    };

    let window = web_sys::window().unwrap();

    if replace {
        window
            .history()
            .unwrap()
            .replace_state_with_url(&JsValue::NULL, "", Some(&hash_url))
            .unwrap();
        // replaceState doesn't fire hashchange, so manually update signals
        let location = use_context::<HashLocation>().expect("HashLocation not provided");
        let (new_path, new_query) = current_hash();
        update_location_signals(&location, new_path, new_query);
    } else {
        // set_hash fires hashchange, which updates signals via listener
        window.location().set_hash(&hash_url).unwrap();
    }
}
