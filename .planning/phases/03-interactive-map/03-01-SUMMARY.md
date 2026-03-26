---
phase: 03-interactive-map
plan: 01
subsystem: ui
tags: [leaflet, wasm-bindgen, openstreetmap, map, circlemarker, popup]

# Dependency graph
requires:
  - phase: 02-spa-foundation
    provides: "Listing page with filter system, School model with coords, URL query param pattern"
provides:
  - "MapView component with Leaflet.js integration via wasm-bindgen"
  - "Color-coded CircleMarker pins with profile-based colors"
  - "Popup HTML with school info and detail page link"
  - "List/map view toggle via URL query param ?view=map"
  - "Filter-reactive marker updates (clear and re-add on change)"
  - "fitBounds auto-zoom on filter change"
affects: [04-travel-time]

# Tech tracking
tech-stack:
  added: [leaflet 0.5.0, js-sys 0.3, Leaflet.js 1.9.4 CDN]
  patterns: [NodeRef-isolated Leaflet initialization, imperative marker management via Effects, JsCast for wasm-bindgen type hierarchy, js_sys::Object+Reflect for JS options]

key-files:
  created:
    - src/pages/map.rs
    - src/components/view_toggle.rs
  modified:
    - Cargo.toml
    - index.html
    - src/pages/listing.rs
    - src/pages/mod.rs
    - src/components/mod.rs
    - public/styles.css

key-decisions:
  - "Used js_sys::Object + Reflect for CircleMarker options instead of serde-wasm-bindgen (avoids extra dependency)"
  - "Map container always in DOM with CSS display toggle (not conditional Leptos render) to prevent Leaflet state loss"
  - "JsCast::unchecked_ref for Layer/Path/CircleMarker type hierarchy traversal"

patterns-established:
  - "NodeRef-isolated JS library integration: Leaflet map initialized once in Effect, mutations via imperative API"
  - "StoredValue for non-reactive Leaflet state (Map instance, markers Vec)"
  - "CSS display:none/block for view switching (preserves JS library DOM state)"

requirements-completed: [MAP-01, MAP-02, MAP-03]

# Metrics
duration: 7min
completed: 2026-03-26
---

# Phase 3 Plan 1: Interactive Map Summary

**Leaflet.js map with color-coded CircleMarker pins, profile popups, filter-synced markers, and list/map view toggle via URL query param**

## Performance

- **Duration:** 7 min
- **Started:** 2026-03-26T19:13:50Z
- **Completed:** 2026-03-26T19:21:04Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- MapView component with full Leaflet integration: map initialization, tile layer, CircleMarker pins with profile-based colors, popup HTML with school info and detail links, fitBounds auto-zoom
- View toggle between list and map views via URL query param (?view=map) with all filters preserved across switches
- CSS styles for map container (500px desktop, 70vh mobile), view toggle buttons, and popup content matching Phase 2 design language

## Task Commits

Each task was committed atomically:

1. **Task 1: Create MapView component with Leaflet integration and view toggle** - `26a5c09` (feat)
2. **Task 2: Integrate map view into listing page with view toggle and CSS** - `98ac576` (feat)

## Files Created/Modified
- `src/pages/map.rs` - MapView component with Leaflet map, CircleMarker pins, popup HTML, fitBounds logic
- `src/components/view_toggle.rs` - ViewToggle component with Liste/Karte buttons
- `src/pages/listing.rs` - Added view toggle, map container, and view param to query string
- `src/pages/mod.rs` - Added `pub mod map;` declaration
- `src/components/mod.rs` - Added `pub mod view_toggle;` declaration
- `Cargo.toml` - Added leaflet 0.5 and js-sys 0.3 dependencies
- `index.html` - Added Leaflet 1.9.4 CDN CSS and JS
- `public/styles.css` - Added map container, view toggle, and popup styles

## Decisions Made
- Used `leaflet` crate 0.5.0 (slowtec/leaflet-rs) for typed wasm-bindgen bindings instead of raw extern blocks
- Constructed CircleMarker options via js_sys::Object + Reflect::set (no serde-wasm-bindgen dependency needed)
- Used JsCast::unchecked_ref to traverse the CircleMarker -> Path -> Layer type hierarchy for add_to/remove/bind_popup methods
- Map div is always rendered in DOM with CSS display toggle, not conditional Leptos rendering (prevents Leaflet map state loss)
- Popup content is plain HTML via format! strings (not Leptos components) since popups live inside Leaflet's DOM

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all functionality is fully wired. Map displays real school coordinates with real profile colors from the school data.

## Next Phase Readiness
- Map integration complete and ready for travel time overlay in Phase 4
- Leaflet map instance accessible for adding route visualization layers
- Valhalla FOSSGIS CORS must still be verified before Phase 4 implementation

## Self-Check: PASSED

- All created files exist (src/pages/map.rs, src/components/view_toggle.rs)
- Commit 26a5c09 found (Task 1)
- Commit 98ac576 found (Task 2)
- cargo check passes with zero errors
- trunk build --release succeeds

---
*Phase: 03-interactive-map*
*Completed: 2026-03-26*
