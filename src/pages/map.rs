use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::models::School;

/// Determine the pin color based on the school's first profile.
/// Matches the exact colors used in school_card.rs for visual consistency.
fn profile_color_for_map(profiles: &[String]) -> &'static str {
    match profiles.first().map(|s| s.as_str()) {
        Some("MINT") => "#22c55e",
        Some("bilingual_english") | Some("bilingual_french") => "#f97316",
        Some("altsprachlich") => "#ef4444",
        Some("music") => "#a855f7",
        Some("sports") => "#3b82f6",
        _ => "#6b7280",
    }
}

/// Human-readable German label for a profile type.
/// Matches school_card.rs profile_label exactly.
fn profile_label_for_map(profile: &str) -> &str {
    match profile {
        "MINT" => "MINT",
        "bilingual_english" => "Bilingual EN",
        "bilingual_french" => "Bilingual FR",
        "altsprachlich" => "Altsprachlich",
        "music" => "Musik",
        "sports" => "Sport",
        "other" => "Sonstiges",
        _ => profile,
    }
}

/// Build popup HTML for a school marker.
/// Uses plain HTML strings (not Leptos components) since the popup lives
/// inside Leaflet's DOM, outside Leptos's reactive tree.
fn build_popup_html(school: &School) -> String {
    let mut html = String::new();
    html.push_str(&format!(
        "<div class='map-popup'><strong>{}</strong>",
        school.name
    ));
    html.push_str(&format!(
        "<br><span class='popup-district'>{}</span>",
        school.district
    ));

    if school.accepts_after_4th_grade == Some(true) {
        html.push_str("<br><span class='popup-grundstaendig'>ab Klasse 5</span>");
    }

    if !school.profile.is_empty() {
        html.push_str("<div class='popup-profiles'>");
        for p in &school.profile {
            let color = profile_color_for_map(&[p.clone()]);
            let label = profile_label_for_map(p);
            html.push_str(&format!(
                "<span style='background:{};color:#fff;padding:1px 6px;\
                 border-radius:8px;font-size:0.7rem;margin:2px;display:inline-block'>{}</span>",
                color, label
            ));
        }
        html.push_str("</div>");
    }

    html.push_str(&format!(
        "<br><a href='/school/{}' class='popup-detail-link'>Details &rarr;</a>",
        school.school_id
    ));
    html.push_str("</div>");
    html
}

/// Build CircleMarker options as a JsValue using js_sys::Object + Reflect.
fn circle_marker_options(color: &str) -> JsValue {
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&obj, &"radius".into(), &JsValue::from_f64(8.0));
    let _ = js_sys::Reflect::set(&obj, &"fillColor".into(), &JsValue::from_str(color));
    let _ = js_sys::Reflect::set(&obj, &"color".into(), &JsValue::from_str("#fff"));
    let _ = js_sys::Reflect::set(&obj, &"weight".into(), &JsValue::from_f64(2.0));
    let _ = js_sys::Reflect::set(&obj, &"opacity".into(), &JsValue::from_f64(1.0));
    let _ = js_sys::Reflect::set(&obj, &"fillOpacity".into(), &JsValue::from_f64(0.85));
    obj.into()
}

/// Interactive map view showing filtered schools as color-coded CircleMarker pins
/// on an OpenStreetMap base layer. Pins are clickable with popup info.
#[component]
pub fn MapView(filtered_schools: Memo<Vec<School>>) -> impl IntoView {
    let map_ref = NodeRef::<leptos::html::Div>::new();

    // Store the Leaflet Map instance (initialized once)
    let map_instance: StoredValue<Option<leaflet::Map>> = StoredValue::new(None);
    // Store current markers for cleanup on filter change
    let markers: StoredValue<Vec<leaflet::CircleMarker>> = StoredValue::new(vec![]);

    // Effect 1: Initialize the map once after mount
    Effect::new(move |_| {
        if map_instance.get_value().is_some() {
            return; // already initialized
        }
        let Some(container) = map_ref.get() else {
            return;
        };

        // Convert Leptos HtmlElement<Div> to web_sys::HtmlElement for leaflet
        let el: &web_sys::HtmlElement = container.unchecked_ref();

        let options = leaflet::MapOptions::default();
        let map = leaflet::Map::new_with_element(el, &options)
            .expect("Leaflet map initialization failed");

        let center = leaflet::LatLng::new(52.52, 13.405);
        map.set_view(&center, 11.0);

        // Add OpenStreetMap tile layer
        let tile_opts = leaflet::TileLayerOptions::new();
        tile_opts.set_attribution(
            "&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors"
                .to_string(),
        );
        tile_opts.set_max_zoom(18.0);
        let tiles = leaflet::TileLayer::new_options(
            "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &tile_opts,
        );
        // TileLayer extends GridLayer extends Layer, so add_to is available via Layer
        let tiles_as_layer: &leaflet::Layer = tiles.unchecked_ref();
        tiles_as_layer.add_to(&map);

        map_instance.set_value(Some(map));
    });

    // Effect 2: Update markers whenever filtered_schools changes
    Effect::new(move |_| {
        let Some(map) = map_instance.get_value() else {
            return;
        };
        let schools = filtered_schools.get();

        // Remove all old markers
        for m in markers.get_value() {
            let layer: &leaflet::Layer = m.unchecked_ref();
            layer.remove();
        }

        let mut new_markers = Vec::new();
        let mut latlngs: Vec<leaflet::LatLng> = Vec::new();

        for school in &schools {
            let Some(coords) = &school.coords else {
                continue;
            };
            let latlng = leaflet::LatLng::new(coords.lat, coords.lng);
            latlngs.push(leaflet::LatLng::new(coords.lat, coords.lng));

            let color = profile_color_for_map(&school.profile);
            let options = circle_marker_options(color);
            let marker = leaflet::CircleMarker::new_with_options(&latlng, &options);

            // Build popup HTML and bind to marker
            let popup_html = build_popup_html(school);
            let marker_as_layer: &leaflet::Layer = marker.unchecked_ref();
            marker_as_layer.bind_popup_with_options(
                &JsValue::from_str(&popup_html),
                &JsValue::NULL,
            );

            // Add marker to map
            marker_as_layer.add_to(&map);
            new_markers.push(marker);
        }

        // Fit bounds to visible markers
        match latlngs.len() {
            0 => {
                // Reset to Berlin default view
                map.set_view(&leaflet::LatLng::new(52.52, 13.405), 11.0);
            }
            1 => {
                map.set_view(&latlngs[0], 14.0);
            }
            _ => {
                let bounds = leaflet::LatLngBounds::new(&latlngs[0], &latlngs[1]);
                for ll in &latlngs[2..] {
                    bounds.extend(ll);
                }
                map.fit_bounds(&bounds);
            }
        }

        markers.set_value(new_markers);
    });

    // The div is ALWAYS in the DOM; parent uses CSS display to show/hide
    view! {
        <div node_ref=map_ref class="map-container"></div>
    }
}
