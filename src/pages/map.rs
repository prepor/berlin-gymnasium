use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::i18n::{profile_label, t, use_language, Language};
use crate::models::School;

/// Build a DivIcon-based Marker for the user's home location,
/// showing a red house icon with a "Mein Standort" label underneath.
fn home_marker_icon() -> leaflet::DivIcon {
    let opts = leaflet::DivIconOptions::new();
    opts.set_html(
        r##"<div class="home-marker">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#dc2626" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>
            <div class="home-marker-label">Mein Standort</div>
        </div>"##.to_string(),
    );
    opts.set_icon_size(leaflet::Point::new(24.0, 40.0));
    opts.set_icon_anchor(leaflet::Point::new(12.0, 24.0));
    opts.set_class_name("home-marker-wrapper".to_string());
    leaflet::DivIcon::new(&opts)
}

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

/// Build popup HTML for a school marker.
/// Uses plain HTML strings (not Leptos components) since the popup lives
/// inside Leaflet's DOM, outside Leptos's reactive tree.
fn build_popup_html(school: &School, lang: Language) -> String {
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
        html.push_str(&format!(
            "<br><span class='popup-grundstaendig'>{}</span>",
            t("from_grade_5", lang)
        ));
    }

    if !school.profile.is_empty() {
        html.push_str("<div class='popup-profiles'>");
        for p in &school.profile {
            let color = profile_color_for_map(&[p.clone()]);
            let label = profile_label(p, lang);
            html.push_str(&format!(
                "<span style='background:{};color:#fff;padding:1px 6px;\
                 border-radius:8px;font-size:0.7rem;margin:2px;display:inline-block'>{}</span>",
                color, label
            ));
        }
        html.push_str("</div>");
    }

    html.push_str(&format!(
        "<br><a href='#/school/{}' class='popup-detail-link'>Details &rarr;</a>",
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
pub fn MapView(
    filtered_schools: Memo<Vec<School>>,
    is_visible: Signal<bool>,
    user_coords: Signal<Option<(f64, f64)>>,
) -> impl IntoView {
    let lang = use_language();
    let map_ref = NodeRef::<leptos::html::Div>::new();

    // Store the Leaflet Map instance (initialized once)
    let map_instance: StoredValue<Option<leaflet::Map>> = StoredValue::new(None);
    // Store current markers for cleanup on filter change
    let markers: StoredValue<Vec<leaflet::CircleMarker>> = StoredValue::new(vec![]);
    // Store home marker separately
    let home_marker: StoredValue<Option<leaflet::Marker>> = StoredValue::new(None);

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

    // Effect 2: Update markers whenever filtered_schools, language, or visibility changes.
    // Subscribes to is_visible so that when the container transitions from hidden to
    // shown we can call invalidateSize (Leaflet needs correct container dimensions)
    // and then place markers / fitBounds with the real viewport.
    Effect::new(move |_| {
        let current_lang = lang.get();
        let visible = is_visible.get();
        let Some(map) = map_instance.get_value() else {
            return;
        };
        let schools = filtered_schools.get();

        // Remove all old markers
        for m in markers.get_value() {
            let layer: &leaflet::Layer = m.unchecked_ref();
            layer.remove();
        }

        // Don't render markers while hidden — Leaflet has 0×0 dimensions
        // and fitBounds/setView would produce wrong results.
        if !visible {
            markers.set_value(vec![]);
            return;
        }

        // Tell Leaflet to recalculate container size (needed after display:none → block)
        map.invalidate_size(false);

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
            let popup_html = build_popup_html(school, current_lang);
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

    // Effect 3: Update home marker when user_coords changes
    Effect::new(move |_| {
        let Some(map) = map_instance.get_value() else {
            return;
        };
        // Remove old home marker
        if let Some(old) = home_marker.get_value() {
            let layer: &leaflet::Layer = old.unchecked_ref();
            layer.remove();
        }

        if let Some((lat, lng)) = user_coords.get() {
            let latlng = leaflet::LatLng::new(lat, lng);
            let icon = home_marker_icon();
            let opts = leaflet::MarkerOptions::new();
            opts.set_icon(icon.unchecked_into());
            let marker = leaflet::Marker::new_with_options(&latlng, &opts);
            let marker_as_layer: &leaflet::Layer = marker.unchecked_ref();
            marker_as_layer.add_to(&map);
            home_marker.set_value(Some(marker));
        } else {
            home_marker.set_value(None);
        }
    });

    // The div is ALWAYS in the DOM; parent uses CSS display to show/hide
    view! {
        <div node_ref=map_ref class="map-container"></div>
    }
}
