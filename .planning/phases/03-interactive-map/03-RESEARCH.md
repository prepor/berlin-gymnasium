# Phase 03: Interactive Map - Research

**Researched:** 2026-03-26
**Domain:** Leaflet.js integration in Leptos/WASM SPA via wasm-bindgen
**Confidence:** HIGH

## Summary

Phase 3 adds an interactive OpenStreetMap map to the existing school listing page, allowing parents to switch between list and map views. All 106 schools have coordinates in the dataset. The map must display color-coded pins by profile type, show popup info on click (school name, district, profiles, grundstandig badge, detail link), and react to filter changes in real time.

The key technical challenge is integrating Leaflet.js (a JavaScript DOM-manipulating library) into a Leptos WASM SPA without DOM ownership conflicts. The `leaflet` crate (v0.5.0, slowtec/leaflet-rs) provides typed wasm-bindgen bindings for Leaflet.js and is the right tool -- it is NOT the same as `leptos-leaflet` (which targets Leptos 0.6 and is ruled out by decision D-01). The map div must be isolated from Leptos's reactive rendering using a NodeRef, and all map mutations (add/remove markers, fit bounds) must happen imperatively through Effects.

**Primary recommendation:** Use the `leaflet` crate (0.5.0) for typed Leaflet.js bindings. Initialize the map once inside `Effect::new` on a NodeRef div. On each filter change, clear and re-add CircleMarkers imperatively. Use `bind_popup_with_options` with `JsValue::from_str` to pass HTML popup content. Load Leaflet 1.9.4 CSS/JS from unpkg CDN in index.html.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: Use raw wasm-bindgen Leaflet.js bindings -- do NOT use leptos-leaflet (0.8 compatibility is unverified, last crates.io release targets Leptos 0.6)
- D-02: Load Leaflet.js 1.9.x via CDN in index.html (link tag for CSS, script tag for JS)
- D-03: Isolate map in a dedicated `<div id="map">` managed via NodeRef -- never let Leptos conditionally render/unmount the map container (Pitfall 7 from PITFALLS.md)
- D-04: Create a Rust MapComponent that initializes Leaflet imperatively via wasm-bindgen/js-sys calls
- D-05: Tab/toggle between list view and map view -- not side-by-side
- D-06: Toggle state stored in URL query param (e.g., `?view=map`) so it's shareable
- D-07: When switching views, filters and sort state are preserved (same URL params)
- D-08: On mobile, map takes full viewport width; on desktop, map sits in the main content area
- D-09: Pins color-coded by primary profile type, matching card badge colors from Phase 2 (MINT=green, music=purple, sports=blue, bilingual=orange, altsprachlich=red, default=gray)
- D-10: Use Leaflet's L.circleMarker or L.divIcon for colored pins (avoid default blue marker for all)
- D-11: No clustering for v1 -- 106 pins is manageable; revisit if map feels cluttered
- D-12: Pin click opens a Leaflet popup showing: school name (bold), district, grundstandig badge (if yes), profile chips
- D-13: Popup includes a "Details" link (`<a>`) navigating to `/school/{school_id}`
- D-14: Popup content is plain HTML generated from Rust (via format! string), not a Leptos component
- D-15: Map centered on Berlin at (52.52, 13.405), zoom level 11 -- shows all 12 districts
- D-16: OpenStreetMap tile layer from tile.openstreetmap.org with attribution
- D-17: Map auto-adjusts bounds (fitBounds) to show all visible pins after filter changes

### Claude's Discretion
- Exact wasm-bindgen binding approach (web-sys vs js_sys::eval vs #[wasm_bindgen] extern blocks)
- Whether to create a separate map.rs module or inline in page component
- Pin icon size and styling details
- Whether to add a "locate me" button (nice-to-have, not required)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MAP-01 | User can view all schools on an interactive OpenStreetMap-based map | `leaflet` crate 0.5.0 provides Map, TileLayer, CircleMarker bindings; Leaflet 1.9.4 CDN for JS/CSS; NodeRef-based initialization pattern |
| MAP-02 | User can click a map pin to see school name and navigate to detail page | `bind_popup_with_options` with `JsValue::from_str` for HTML content; popup contains `<a href="/school/{id}">` link |
| MAP-03 | Map reflects active filters (only shows filtered schools) | Reuse existing `filter_and_sort` from listing.rs; Effect watches filtered_schools Memo and imperatively clears/re-adds markers |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- Tech stack: Rust + Leptos -- static SPA compiled to WASM
- Deployment: GitHub Pages -- no server-side rendering or API
- Data format: YAML files in data/schools/, compiled to JSON at build time via build.rs
- Build tool: trunk
- The project already uses Leptos 0.8, wasm-bindgen 0.2, web-sys 0.3, serde 1, serde_json 1
- GSD workflow enforcement: edits go through GSD commands

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| leaflet (crate) | 0.5.0 | Typed wasm-bindgen bindings for Leaflet.js | Maintained wrapper (slowtec GmbH); provides Map, TileLayer, CircleMarker, Popup, DivIcon, LatLngBounds, fitBounds; uses wasm-bindgen 0.2.103 (compatible with project's 0.2); NOT the same as leptos-leaflet |
| Leaflet.js | 1.9.4 | Map rendering engine loaded via CDN | Industry standard; ~40KB; raster tiles; no WebGL needed; 106 markers trivial |
| OpenStreetMap tiles | - | Free map tile provider | Free with attribution; `https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png` |

### Supporting (already in project)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| wasm-bindgen | 0.2 | Rust-JS FFI (already in Cargo.toml) | JsValue::from_str for popup HTML content |
| web-sys | 0.3 | Browser DOM API access (already in Cargo.toml) | NodeRef::get returns web_sys::HtmlElement for map container |
| js-sys | 0.3 | JavaScript builtins (may need to add) | js_sys::Array for LatLngBounds::new_from_list |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `leaflet` crate | Raw #[wasm_bindgen] extern blocks | More boilerplate, no typed bindings, must maintain manually; leaflet crate does this already |
| `leaflet` crate | leptos-leaflet | Last release targets Leptos 0.6; 0.8 compat unverified; RULED OUT by D-01 |
| CircleMarker | DivIcon | DivIcon allows richer HTML styling (e.g., number labels) but CircleMarker is simpler for solid-color dots; D-10 allows either |
| unpkg CDN | cdnjs or jsdelivr | All equivalent; unpkg is simplest URL pattern |

**Installation:**
Add to Cargo.toml `[dependencies]`:
```toml
leaflet = "0.5"
js-sys = "0.3"
```

Add to `web-sys` features list: `"Element"` (if not already present for NodeRef).

Add to index.html `<head>`:
```html
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
  integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
  crossorigin="" />
```

Add before `</body>` in index.html:
```html
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
  integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
  crossorigin=""></script>
```

## Architecture Patterns

### Recommended Project Structure
```
src/
  pages/
    listing.rs          # MODIFY: add view toggle, extract shared filter logic
    map.rs              # NEW: MapComponent with Leaflet integration
    detail.rs           # unchanged
    mod.rs              # add map module
  components/
    filter_panel.rs     # unchanged (reused by both views)
    view_toggle.rs      # NEW: list/map toggle button
    mod.rs              # add view_toggle module
  models.rs             # unchanged (School has coords)
  state.rs              # unchanged
  app.rs                # unchanged (no new route; map is a view within listing)
```

### Pattern 1: NodeRef-Isolated Map Initialization

**What:** The map div exists in the DOM permanently (CSS show/hide, not conditional Leptos render). A Leptos Effect initializes Leaflet on it once after mount. All subsequent map operations (add markers, fit bounds) are imperative calls through the `leaflet` crate, not reactive Leptos rendering.

**When to use:** Any time a JS library that owns DOM state is used inside a Leptos component.

**Example:**
```rust
use leptos::prelude::*;
use leaflet::{Map, MapOptions, TileLayer, TileLayerOptions, LatLng, CircleMarker, LatLngBounds};
use wasm_bindgen::prelude::*;

#[component]
pub fn MapView(
    filtered_schools: Memo<Vec<School>>,
) -> impl IntoView {
    let map_ref = NodeRef::<leptos::html::Div>::new();
    // Store the Leaflet Map instance for later use
    let map_instance: StoredValue<Option<Map>> = StoredValue::new(None);

    // Initialize map once after mount
    Effect::new(move |_| {
        if map_instance.get_value().is_some() {
            return; // already initialized
        }
        let Some(container) = map_ref.get() else { return };

        let options = MapOptions::default();
        let map = Map::new_with_element(
            &container.into_any().into(),  // HtmlElement
            &options,
        ).expect("map init failed");

        let center = LatLng::new(52.52, 13.405);
        map.set_view(&center, 11.0);

        // Add tile layer
        let tile_opts = TileLayerOptions::new();
        tile_opts.set_attribution(
            "&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a>"
                .to_string(),
        );
        tile_opts.set_max_zoom(18.0);
        let tiles = TileLayer::new_options(
            "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &tile_opts,
        );
        tiles.add_to(&map);

        map_instance.set_value(Some(map));
    });

    // The div is ALWAYS in the DOM; parent uses CSS display to show/hide
    view! {
        <div node_ref=map_ref class="map-container" id="map-view"></div>
    }
}
```

### Pattern 2: Imperative Marker Update on Filter Change

**What:** A separate Effect watches the filtered_schools Memo. On each change, it removes all existing markers and re-creates them from the filtered list. This avoids complex diffing of marker sets.

**When to use:** When the filtered set changes via reactive signals but markers are imperative JS objects.

**Example:**
```rust
// Store markers for cleanup
let markers: StoredValue<Vec<CircleMarker>> = StoredValue::new(vec![]);

Effect::new(move |_| {
    let Some(map) = map_instance.get_value() else { return };
    let schools = filtered_schools.get();

    // Remove old markers
    for m in markers.get_value() {
        m.remove();
    }

    let mut new_markers = Vec::new();
    let mut latlngs: Vec<LatLng> = Vec::new();

    for school in &schools {
        let Some(coords) = &school.coords else { continue };
        let latlng = LatLng::new(coords.lat, coords.lng);
        latlngs.push(LatLng::new(coords.lat, coords.lng));

        let color = profile_color_for_map(&school.profile);
        let options = serde_wasm_bindgen::to_value(&serde_json::json!({
            "radius": 8,
            "fillColor": color,
            "color": "#fff",
            "weight": 2,
            "opacity": 1.0,
            "fillOpacity": 0.85,
        })).unwrap();
        let marker = CircleMarker::new_with_options(&latlng, &options);

        // Build popup HTML
        let popup_html = build_popup_html(school);
        marker.bind_popup_with_options(
            &JsValue::from_str(&popup_html),
            &JsValue::NULL,
        );

        marker.add_to(&map);
        new_markers.push(marker);
    }

    // Fit bounds to visible markers
    if latlngs.len() >= 2 {
        let bounds = LatLngBounds::new(&latlngs[0], &latlngs[1]);
        for ll in &latlngs[2..] {
            bounds.extend(ll);
        }
        map.fit_bounds(&bounds);
    } else if latlngs.len() == 1 {
        map.set_view(&latlngs[0], 14.0);
    }

    markers.set_value(new_markers);
});
```

### Pattern 3: View Toggle via URL Query Param

**What:** The listing page reads a `view` query param. When `view=map`, it shows the map container and hides the school grid. When absent or `view=list`, it shows the grid and hides the map. Toggle is a button/tab that navigates with the param, preserving all other query params (filters, sort).

**When to use:** Per decisions D-05, D-06, D-07.

**Example:**
```rust
// Read view mode from URL
let is_map_view = Signal::derive(move || {
    query.read().get("view").as_deref() == Some("map")
});

// Toggle button
let nav = use_navigate();
let on_toggle_view = move |_| {
    let current_qs = /* rebuild current query string */;
    let new_view = if is_map_view.get() { "" } else { "map" };
    // Update only the view param, keep everything else
    nav(&format!("/?{}&view={}", current_qs, new_view), ...);
};

// CSS-based show/hide (not conditional render!)
view! {
    <div class="map-container" style:display=move || {
        if is_map_view.get() { "block" } else { "none" }
    }>
        <MapView filtered_schools=filtered_schools />
    </div>
    <section class="school-grid" style:display=move || {
        if is_map_view.get() { "none" } else { "" }
    }>
        /* ... existing school cards ... */
    </section>
}
```

### Pattern 4: Popup HTML Generation

**What:** Build popup content as a plain HTML string in Rust using format!. This is per D-14 -- no Leptos component rendering for popups.

**Example:**
```rust
fn build_popup_html(school: &School) -> String {
    let mut html = String::new();
    html.push_str(&format!(
        "<div class='map-popup'><strong>{}</strong>",
        school.name
    ));
    html.push_str(&format!("<br><span class='popup-district'>{}</span>", school.district));

    if school.accepts_after_4th_grade == Some(true) {
        html.push_str("<br><span class='popup-grundstaendig'>ab Klasse 5</span>");
    }

    if !school.profile.is_empty() {
        html.push_str("<div class='popup-profiles'>");
        for p in &school.profile {
            let color = profile_color_for_map(&[p.clone()]);
            let label = profile_label(p);
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
```

### Anti-Patterns to Avoid

- **Conditional rendering of map container:** NEVER use `{move || if show_map { view!{<div>...map...</div>} } else { view!{} }}`. This destroys the Leaflet map instance on every toggle. Use CSS `display: none/block` instead.
- **Reactive markers via `<For>`:** Do NOT try to render markers as Leptos components. Leaflet markers are JS objects managed by Leaflet, not DOM elements in Leptos's reactive tree.
- **Re-initializing map on every filter change:** The map should be created ONCE. Only markers are updated on filter changes. Guard the init Effect to run only once.
- **Using Leptos view! for popup content:** Popup HTML lives inside Leaflet's DOM, outside Leptos. Use `format!` strings, not `view!` macros.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Leaflet JS bindings | Custom #[wasm_bindgen] extern blocks for L.map, L.marker, L.tileLayer | `leaflet` crate 0.5.0 | Already has typed bindings for Map, Marker, CircleMarker, TileLayer, Popup, LatLngBounds, DivIcon; maintained by slowtec GmbH |
| Map tile serving | Self-hosted tile server | OpenStreetMap public tile CDN | Free, reliable, attribution-only requirement |
| Marker clustering | Manual viewport culling | Skip for v1 (D-11) | 106 markers is trivially renderable by Leaflet |
| Filter logic | Separate filter implementation for map view | Reuse `filter_and_sort` from listing.rs | Exact same filtering; extract to shared module or pass filtered Memo |

**Key insight:** The `leaflet` crate eliminates the main risk of this phase (fragile JS interop). It provides typed Rust APIs for every Leaflet operation needed. The only "raw" JS interaction needed is passing HTML strings as `JsValue::from_str` for popup content.

## Common Pitfalls

### Pitfall 1: Leptos Re-renders Destroy Leaflet Map Container
**What goes wrong:** Using conditional Leptos rendering (`Show`, `if/else in view!`) around the map div causes Leaflet to lose its map instance when toggling views.
**Why it happens:** Leaflet stores state (layers, zoom, markers) on the DOM element. When Leptos removes the element from the DOM, all that state is lost.
**How to avoid:** Keep the map `<div>` always in the DOM. Use `style:display` to show/hide. Initialize the map once in an Effect guarded by a "already initialized" check.
**Warning signs:** Map goes blank or resets to default view when switching between list and map views.

### Pitfall 2: Effect Runs Before DOM Node Exists
**What goes wrong:** The init Effect tries to call `Map::new_with_element` before the `<div>` is in the DOM.
**Why it happens:** Effects run on the next microtask after component creation, but if the component itself hasn't rendered yet (e.g., inside a suspended boundary), the NodeRef is empty.
**How to avoid:** Always check `if let Some(container) = map_ref.get()` before initializing. In CSR mode with Leptos 0.8, the div is available on the first Effect tick.
**Warning signs:** `expect("map init failed")` panics on first load.

### Pitfall 3: CircleMarker Options Passed as Wrong JsValue Type
**What goes wrong:** `CircleMarker::new_with_options` takes `&JsValue` for options. If you pass a Rust struct directly or an incorrectly serialized value, the marker renders with wrong defaults or panics.
**Why it happens:** The `leaflet` crate's options APIs accept generic `JsValue`, not typed Rust structs.
**How to avoid:** Use `serde_wasm_bindgen::to_value` or manually construct a `js_sys::Object` with the properties. Test with simple values first.
**Warning signs:** Markers appear but with default blue color or wrong radius.

### Pitfall 4: Popup Link Navigation Bypasses Leptos Router
**What goes wrong:** The `<a href="/school/{id}">` link inside a Leaflet popup triggers a full page navigation instead of client-side routing.
**Why it happens:** Leaflet's popup DOM is outside Leptos's event delegation. Leptos router's link interception only works on elements managed by Leptos.
**How to avoid:** Two options: (a) Accept the full-page nav -- it works, just reloads the WASM app on the detail page; (b) Add a click event listener on the marker/popup that calls `use_navigate()` programmatically instead of using an `<a>` tag. Option (a) is simpler and acceptable for v1 given the SPA rehydrates quickly.
**Warning signs:** Clicking "Details" link in popup causes a white flash as the full page reloads instead of smooth SPA transition.

### Pitfall 5: fitBounds on Empty Marker Set Throws
**What goes wrong:** When all schools are filtered out, calling `fitBounds` with an empty or single-point bounds causes a Leaflet error.
**Why it happens:** `LatLngBounds` requires at least two distinct points; an empty filtered set has zero.
**How to avoid:** Guard fitBounds: if 0 markers, reset to Berlin default view (52.52, 13.405, zoom 11). If 1 marker, use `set_view` at zoom 14 instead of `fitBounds`. If 2+ markers, use fitBounds normally.
**Warning signs:** JS console error `Bounds are not valid` when aggressive filters return 0 or 1 school.

### Pitfall 6: serde_wasm_bindgen Dependency Not in Cargo.toml
**What goes wrong:** Code uses `serde_wasm_bindgen::to_value` for CircleMarker options but the crate isn't declared as a dependency.
**Why it happens:** Easy to forget this bridging crate which connects serde types to JsValue.
**How to avoid:** Add `serde-wasm-bindgen = "0.6"` to Cargo.toml if using this approach. Alternative: construct options with `js_sys::Object` and `js_sys::Reflect::set` (no extra dependency but more verbose).
**Warning signs:** Compilation error: `unresolved import serde_wasm_bindgen`.

## Code Examples

### Profile Color Mapping (reuse from school_card.rs)
```rust
// Source: src/components/school_card.rs (existing)
// These exact colors match Phase 2 badge colors per D-09
fn profile_color_for_map(profiles: &[String]) -> &'static str {
    // Use first profile for pin color
    match profiles.first().map(|s| s.as_str()) {
        Some("MINT") => "#22c55e",         // green
        Some("bilingual_english") | Some("bilingual_french") => "#f97316", // orange
        Some("altsprachlich") => "#ef4444", // red
        Some("music") => "#a855f7",        // purple
        Some("sports") => "#3b82f6",       // blue
        _ => "#6b7280",                     // gray
    }
}
```

### CSS for Map Container and View Toggle
```css
/* Map container - always in DOM, shown/hidden via style:display */
.map-container {
    width: 100%;
    height: 500px;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

@media (max-width: 767px) {
    .map-container {
        height: 70vh;
        border-radius: 0;
    }
}

/* View toggle tabs */
.view-toggle {
    display: inline-flex;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    overflow: hidden;
}

.view-toggle-btn {
    padding: 6px 16px;
    border: none;
    background: #fff;
    color: #64748b;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.15s;
}

.view-toggle-btn.active {
    background: #2563eb;
    color: #fff;
}

/* Popup styling */
.map-popup strong {
    font-size: 0.95rem;
    color: #1e293b;
}

.popup-district {
    color: #64748b;
    font-size: 0.8rem;
}

.popup-grundstaendig {
    background: #0d9488;
    color: #fff;
    padding: 1px 6px;
    border-radius: 8px;
    font-size: 0.7rem;
    font-weight: 600;
}

.popup-detail-link {
    color: #2563eb;
    font-size: 0.85rem;
    font-weight: 500;
}
```

### Leaflet CDN Tags for index.html
```html
<!-- In <head>, after existing CSS -->
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
  integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
  crossorigin="" />

<!-- Before </body> -->
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
  integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
  crossorigin=""></script>
```

### Query Param Integration for View Toggle
```rust
// Source: adapted from existing listing.rs URL param pattern

// In build_query_string, add view param:
fn build_query_string(/* existing params */ view: &str) -> String {
    let mut params = Vec::new();
    // ... existing filter params ...
    if view == "map" {
        params.push("view=map".to_string());
    }
    params.join("&")
}

// Read from query:
let is_map_view = Signal::derive(move || {
    query.read().get("view").as_deref() == Some("map")
});
```

### CircleMarker Options via js_sys (no serde-wasm-bindgen needed)
```rust
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

fn circle_marker_options(color: &str) -> JsValue {
    let obj = Object::new();
    Reflect::set(&obj, &"radius".into(), &JsValue::from_f64(8.0)).unwrap();
    Reflect::set(&obj, &"fillColor".into(), &JsValue::from_str(color)).unwrap();
    Reflect::set(&obj, &"color".into(), &JsValue::from_str("#fff")).unwrap();
    Reflect::set(&obj, &"weight".into(), &JsValue::from_f64(2.0)).unwrap();
    Reflect::set(&obj, &"opacity".into(), &JsValue::from_f64(1.0)).unwrap();
    Reflect::set(&obj, &"fillOpacity".into(), &JsValue::from_f64(0.85)).unwrap();
    obj.into()
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| leaflet crate 0.4 | leaflet crate 0.5.0 | Sep 2025 | New API with wasm-bindgen 0.2.103; edition 2024 support |
| leptos-leaflet (for Leptos map integration) | leaflet crate + raw NodeRef | Leptos 0.8 release | leptos-leaflet lags behind at Leptos 0.6; direct leaflet crate is the correct approach now |
| serde_yaml for build step | serde-saphyr | 2023 | Already adopted in Phase 1; no change needed |

**Deprecated/outdated:**
- `leptos-leaflet` (crates.io): Last release targets Leptos 0.6. Not compatible with Leptos 0.8. Do not use.
- `leaflet-rs` (thburghout fork): Mirror/fork, not canonical. Use slowtec/leaflet-rs via `leaflet` crate.

## Open Questions

1. **NodeRef type for generic div in Leptos 0.8**
   - What we know: `NodeRef::<leptos::html::Div>::new()` should work for a plain div element. The Effect returns `Option<HtmlElement<Div>>` which derefs to `web_sys::HtmlDivElement`.
   - What's unclear: Exact cast needed to go from `leptos::html::HtmlElement<Div>` to `&web_sys::HtmlElement` for `Map::new_with_element`. May need `.deref()` or `JsCast::unchecked_ref()`.
   - Recommendation: Test the cast at implementation time. If `HtmlElement<Div>` doesn't deref directly to `web_sys::HtmlElement`, use `wasm_bindgen::JsCast::unchecked_ref::<web_sys::HtmlElement>()`.

2. **Popup link navigation: full reload vs SPA transition**
   - What we know: Leaflet popup `<a>` links will trigger full page reload (Pitfall 4). The WASM app reloads in ~1-2 seconds.
   - What's unclear: Whether users will find the reload jarring.
   - Recommendation: Ship with `<a href>` links (simplest). If reload is noticeable, add a click handler on the popup that calls `window.location.hash` or Leptos navigate programmatically in a future iteration.

3. **Whether js-sys is needed as an explicit dependency**
   - What we know: `leaflet` crate 0.5.0 depends on `js-sys = "0.3.80"`. If we use `js_sys::Object` / `js_sys::Reflect` for marker options, we need it in our code.
   - What's unclear: Whether `leaflet` re-exports js-sys types or if we need a direct dependency.
   - Recommendation: Add `js-sys = "0.3"` to Cargo.toml dependencies explicitly. It's a zero-cost addition since leaflet already pulls it in.

## Sources

### Primary (HIGH confidence)
- [leaflet crate 0.5.0 docs.rs](https://docs.rs/leaflet/0.5.0/leaflet/) - Map, CircleMarker, TileLayer, Popup, LatLngBounds API
- [leaflet-rs GitHub (slowtec)](https://github.com/slowtec/leaflet-rs) - Source code, dependency versions, examples
- [Leptos Book: web_sys integration](https://book.leptos.dev/web_sys.html) - NodeRef usage, Effect timing, HtmlElement deref patterns
- [Leaflet.js 1.9.4 download page](https://leafletjs.com/download.html) - CDN URLs with SRI hashes

### Secondary (MEDIUM confidence)
- [Leaflet.js reference docs](https://leafletjs.com/reference.html) - CircleMarker options, Popup options, fitBounds behavior
- [Leaflet Quick Start](https://leafletjs.com/examples/quick-start/) - CDN integrity hashes verified

### Tertiary (LOW confidence)
- Popup link navigation behavior (full reload vs SPA) - Inferred from DOM ownership model; needs runtime verification

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - `leaflet` crate 0.5.0 verified on docs.rs with full API inspection; Leaflet 1.9.4 is current stable
- Architecture: HIGH - NodeRef isolation pattern is documented in Leptos book and is the standard approach for JS library integration; marker update pattern follows standard Leaflet imperative API
- Pitfalls: HIGH - DOM ownership conflict is well-documented (Pitfall 7 in PITFALLS.md); fitBounds edge cases are known Leaflet behavior

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable domain; Leaflet and leaflet crate are mature)
