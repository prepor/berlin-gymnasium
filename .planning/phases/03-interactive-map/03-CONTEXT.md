# Phase 3: Interactive Map - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Interactive OpenStreetMap map with clickable pins for all Berlin Gymnasien, synced to active filters from the listing page. Parents can switch between list and map views, click pins to see school info, and navigate to detail pages.

Requirements: MAP-01, MAP-02, MAP-03.

</domain>

<decisions>
## Implementation Decisions

### Map Library
- **D-01:** Use raw wasm-bindgen Leaflet.js bindings — do NOT use leptos-leaflet (0.8 compatibility is unverified, last crates.io release targets Leptos 0.6)
- **D-02:** Load Leaflet.js 1.9.x via CDN in index.html (link tag for CSS, script tag for JS)
- **D-03:** Isolate map in a dedicated `<div id="map">` managed via NodeRef — never let Leptos conditionally render/unmount the map container (Pitfall 7 from PITFALLS.md)
- **D-04:** Create a Rust MapComponent that initializes Leaflet imperatively via wasm-bindgen/js-sys calls

### Map-Listing Integration
- **D-05:** Tab/toggle between list view and map view — not side-by-side
- **D-06:** Toggle state stored in URL query param (e.g., `?view=map`) so it's shareable
- **D-07:** When switching views, filters and sort state are preserved (same URL params)
- **D-08:** On mobile, map takes full viewport width; on desktop, map sits in the main content area

### Pin Styling
- **D-09:** Pins color-coded by primary profile type, matching card badge colors from Phase 2 (MINT=green, music=purple, sports=blue, bilingual=orange, altsprachlich=red, default=gray)
- **D-10:** Use Leaflet's L.circleMarker or L.divIcon for colored pins (avoid default blue marker for all)
- **D-11:** No clustering for v1 — 106 pins is manageable; revisit if map feels cluttered

### Popup Content
- **D-12:** Pin click opens a Leaflet popup showing: school name (bold), district, grundständig badge (if yes), profile chips
- **D-13:** Popup includes a "Details" link (`<a>`) navigating to `/school/{school_id}`
- **D-14:** Popup content is plain HTML generated from Rust (via format! string), not a Leptos component

### Default Map View
- **D-15:** Map centered on Berlin at (52.52, 13.405), zoom level 11 — shows all 12 districts
- **D-16:** OpenStreetMap tile layer from tile.openstreetmap.org with attribution
- **D-17:** Map auto-adjusts bounds (fitBounds) to show all visible pins after filter changes

### Claude's Discretion
- Exact wasm-bindgen binding approach (web-sys vs js_sys::eval vs #[wasm_bindgen] extern blocks)
- Whether to create a separate map.rs module or inline in page component
- Pin icon size and styling details
- Whether to add a "locate me" button (nice-to-have, not required)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Technology
- `.planning/research/STACK.md` — leptos-leaflet status, Leaflet.js recommendation, OpenStreetMap tiles
- `.planning/research/PITFALLS.md` — Pitfall 7: Map/WASM DOM ownership conflict (MUST isolate in NodeRef)
- `CLAUDE.md` — Leaflet.js 1.9.x via CDN, OpenStreetMap tiles, leptos-leaflet backup plan

### Existing Code (Phase 2)
- `src/app.rs` — Router setup, existing routes
- `src/pages/listing.rs` — Filter logic, URL query param handling (reuse for map view)
- `src/models.rs` — School struct with coords (lat/lng), profile types
- `src/state.rs` — AppState with schools vec
- `src/components/filter_panel.rs` — Filter UI (shared between list and map views)
- `public/styles.css` — Existing styles to extend
- `index.html` — Where to add Leaflet CDN links

### Data Contract
- `data/schools/01P03.yaml` — Example school file showing coords format (`coords: {lat: 52.52, lng: 13.39}`)

### Requirements
- `.planning/REQUIREMENTS.md` — MAP-01, MAP-02, MAP-03

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/pages/listing.rs` — Filter logic (Memo-based filtering, URL param read/write) can be shared with map view
- `src/components/filter_panel.rs` — Filter sidebar component, reusable as-is for map view
- `src/components/filter_chips.rs` — Active filter display, reusable
- `src/models.rs` — School struct already has `coords: Option<Coords>` with lat/lng
- `src/state.rs` — AppState provides schools via context

### Established Patterns
- URL query params for filter state (use_query_map + use_navigate)
- Memo-based reactive filtering
- German-language UI labels
- Mobile-first responsive CSS with media queries

### Integration Points
- `src/app.rs` — No new route needed; map is a view toggle on the listing page, not a separate route
- `index.html` — Add Leaflet CDN (CSS + JS) in head/body
- `public/styles.css` — Add map container and toggle button styles

</code_context>

<specifics>
## Specific Ideas

- The map view and list view share the same filtered school set — changing a filter updates both simultaneously
- Profile badge colors from Phase 2 SchoolCard should be reused for pin colors (visual consistency)
- Grundständig flag should be visible on map popups — it's a primary differentiator for parents
- Berlin's geography means some districts (Mitte, Charlottenburg) have many overlapping schools — zoom interaction is important

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 03-interactive-map*
*Context gathered: 2026-03-26*
