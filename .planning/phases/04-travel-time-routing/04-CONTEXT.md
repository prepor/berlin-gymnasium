# Phase 4: Travel Time Routing - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

User address input with geocoding, travel time calculation to all schools via Valhalla matrix API, and sort-by-commute. Parents enter their home address, see travel times (walk/transit/car) for each school, and can sort the listing by commute time.

Requirements: TRVL-01, TRVL-02, TRVL-03.

</domain>

<decisions>
## Implementation Decisions

### Geocoding
- **D-01:** Use Photon (photon.komoot.io) for address geocoding — CORS-enabled, no API key, browser-safe
- **D-02:** Do NOT use Nominatim directly from browser (autocomplete prohibited, User-Agent requirement — Pitfall 9)
- **D-03:** Geocode request format: `https://photon.komoot.io/api/?q={address}&lat=52.52&lon=13.405&limit=5&lang=de` (biased toward Berlin)
- **D-04:** Return top 5 suggestions for user to pick from (dropdown after typing)

### Address Input UX
- **D-05:** Text input with submit button — user types address, hits Enter or clicks button
- **D-06:** After submit, show top 5 Photon results as a dropdown for disambiguation
- **D-07:** Selected address stored in URL query param (`?from=lat,lng`) so it's shareable
- **D-08:** "Clear address" button to remove travel time data and return to normal listing
- **D-09:** German UI labels: "Adresse eingeben", "Fahrzeit berechnen", "Adresse löschen"

### Travel Time API
- **D-10:** Use Valhalla FOSSGIS server: `https://valhalla1.openstreetmap.de/sources_to_targets`
- **D-11:** Matrix endpoint: one source (user address) → N targets (all filtered school coords) in a single API call
- **D-12:** Three costing modes in parallel requests: `pedestrian` (walking), `multimodal` (transit), `auto` (car)
- **D-13:** Parse response `sources_to_targets[0][i].time` (seconds) for each school
- **D-14:** If Valhalla CORS fails at runtime, show error message "Reisezeit-Berechnung nicht verfügbar" and gracefully degrade (listing still works without travel time)

### Travel Time Display
- **D-15:** When address is set, each school card shows a travel time row: 🚶 Xmin | 🚌 Ymin | 🚗 Zmin
- **D-16:** Travel times rounded to nearest minute
- **D-17:** Sort-by-travel-time added as a sort option (walk, transit, or car — user picks which mode to sort by)
- **D-18:** Schools without coords (if any) show "Keine Fahrzeit verfügbar"

### Performance & Error Handling
- **D-19:** Debounce geocoding requests by 500ms after user stops typing
- **D-20:** Cache travel time results per address (in-memory HashMap<String, Vec<TravelTime>>)
- **D-21:** Show loading spinner on cards while travel time is being fetched
- **D-22:** If individual school has no route (e.g., island), show "—" instead of time

### Claude's Discretion
- Exact wasm-bindgen fetch implementation (web-sys Fetch API vs gloo-net)
- Whether to create a separate travel_time.rs module or inline in listing
- Spinner/loading indicator styling
- Whether Photon suggestions appear as overlay dropdown or inline list

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Technology
- `.planning/research/STACK.md` — Valhalla FOSSGIS details, Photon/Nominatim guidance
- `.planning/research/PITFALLS.md` — Pitfall 3 (OSRM unusable), Pitfall 9 (Nominatim browser restrictions)
- `CLAUDE.md` — Valhalla API, Nominatim, routing section

### Existing Code (Phase 2+3)
- `src/pages/listing.rs` — Filter logic, URL query params, sort controls, card rendering
- `src/models.rs` — School struct with coords, SortField enum (extend with travel time)
- `src/state.rs` — AppState
- `src/components/school_card.rs` — Card component (add travel time row)
- `src/components/sort_controls.rs` — Sort dropdown (add travel time sort options)
- `src/components/filter_panel.rs` — Filter sidebar
- `src/pages/map.rs` — MapView (may show travel time radius circle later)
- `public/styles.css` — Existing styles
- `Cargo.toml` — Dependencies

### Requirements
- `.planning/REQUIREMENTS.md` — TRVL-01, TRVL-02, TRVL-03

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/pages/listing.rs` — URL query param handling (use_query_map + use_navigate) for filter state — extend with `from` param
- `src/models.rs` — SortField enum to extend with TravelTimeWalk/Transit/Car
- `src/components/sort_controls.rs` — Sort dropdown to extend with travel time options
- `src/components/school_card.rs` — Card component to extend with travel time row

### Established Patterns
- URL query params for all persisted state
- Memo-based reactive filtering and sorting
- German-language UI labels
- `web-sys` and `js-sys` for browser API access (already in Cargo.toml from Phase 3)
- wasm-bindgen for JS interop

### Integration Points
- `src/pages/listing.rs` — Add address input component, pass travel time data to cards and sort
- `src/components/school_card.rs` — Add conditional travel time row
- `src/components/sort_controls.rs` — Add travel time sort options
- `src/models.rs` — Add TravelTime struct and SortField variants

</code_context>

<specifics>
## Specific Ideas

- Valhalla `sources_to_targets` can compute all 106 travel times in ONE API call per mode — very efficient
- Three parallel requests (pedestrian, multimodal, auto) can fire simultaneously via spawn_local
- Address input should feel like a search bar — prominent, above the filter panel
- Travel time sorting should default to transit (most relevant for Berlin parents — public transport is primary)
- Cache is per-session only (in-memory) — no localStorage persistence needed

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-travel-time-routing*
*Context gathered: 2026-03-26*
