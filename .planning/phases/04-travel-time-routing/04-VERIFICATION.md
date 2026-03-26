---
phase: 04-travel-time-routing
verified: 2026-03-26T20:29:27Z
status: human_needed
score: 7/7 must-haves verified (automated)
human_verification:
  - test: "Type an address (e.g. 'Alexanderplatz') into the address input and verify geocoded suggestions appear"
    expected: "After ~500ms debounce, a dropdown with up to 5 Berlin-biased suggestions appears from Photon API"
    why_human: "Requires running application and live Photon API; CORS behavior and response format can only be verified at runtime"
  - test: "Select a suggestion and verify travel times appear on all school cards"
    expected: "Walk/bike/car travel times in minutes display on each card; loading state shown during fetch"
    why_human: "Requires live Valhalla API calls with POST+GET fallback; network behavior and CORS untestable without browser"
  - test: "Sort by 'Fahrzeit (zu Fuss)' and verify schools reorder by walking time"
    expected: "Schools reorder ascending by walking minutes; schools without times sort last"
    why_human: "Requires populated travel time data from live API to verify sort order"
  - test: "Verify URL contains ?from=lat,lng after selecting address; open URL in new tab to test shareability"
    expected: "URL has from= param; opening in new tab auto-loads travel times"
    why_human: "Browser-based navigation and URL persistence testing"
  - test: "Click 'Adresse loeschen' and verify travel times disappear and from= removed from URL"
    expected: "Travel time rows vanish from cards; URL no longer has from= param"
    why_human: "Interactive UI state transition"
  - test: "Check mobile responsiveness of address input at phone width"
    expected: "Address input scales to full width; suggestion dropdown remains usable"
    why_human: "Visual layout verification"
---

# Phase 4: Travel Time Routing Verification Report

**Phase Goal:** Parents can enter their home address and see how long it takes to reach each school by walking, cycling, and driving -- and sort schools by commute time
**Verified:** 2026-03-26T20:29:27Z
**Status:** human_needed
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Photon geocoding function accepts a query string and returns parsed address suggestions with lat/lng | VERIFIED | `src/services/geocoding.rs` lines 63-80: `geocode_address` async fn calls `photon.komoot.io/api/` with Berlin bias params, returns `Vec<PhotonFeature>` with `lat()`/`lng()` helpers |
| 2 | Valhalla matrix function accepts origin coords + target coords + costing mode and returns travel times in minutes | VERIFIED | `src/services/routing.rs` lines 22-85: `fetch_travel_times_matrix` with POST+GET CORS fallback, seconds-to-minutes rounding |
| 3 | TravelTimes struct holds walk/bike/car minutes per school | VERIFIED | `src/models.rs` lines 125-130: `TravelTimes { walk_minutes, bike_minutes, car_minutes }` all `Option<u32>` |
| 4 | SortField enum includes travel time sort variants | VERIFIED | `src/models.rs` lines 134-142: `TravelTimeWalk`, `TravelTimeBike`, `TravelTimeCar` variants with `from_query`/`to_query`/`is_travel_time` |
| 5 | User can type an address and see geocoded suggestions in a dropdown | VERIFIED | `src/components/address_input.rs`: 153-line component with 500ms debounced `use_debounce_fn_with_arg`, calls `geocode_address`, renders `<ul class="address-suggestions">` with clickable items |
| 6 | School cards show walking, cycling, and car travel time in minutes when address is set | VERIFIED | `src/components/school_card.rs` lines 91-140: conditional travel time row with emoji walk/bike/car display, loading state "Berechne Fahrzeit...", em-dash for unreachable, "Keine Fahrzeit verfuegbar" for no coords |
| 7 | User can sort schools by any of the three travel time modes | VERIFIED | `src/pages/listing.rs` lines 158-206: `filter_and_sort` handles all three travel sort variants with proper None-sorts-last logic; `src/components/sort_controls.rs` lines 31-42: conditional travel time options shown when `has_travel_times` is true |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `src/services/geocoding.rs` | Photon API integration | VERIFIED | 80 lines; exports `geocode_address`, `PhotonFeature`; uses `photon.komoot.io` with Berlin bias |
| `src/services/routing.rs` | Valhalla matrix API integration | VERIFIED | 125 lines; exports `fetch_travel_times_matrix`, `fetch_all_travel_times`; POST+GET fallback; seconds-to-minutes conversion |
| `src/services/mod.rs` | Module declarations | VERIFIED | Declares `pub mod geocoding;` and `pub mod routing;` |
| `src/models.rs` | TravelTimes struct, extended SortField | VERIFIED | `TravelTimes` at line 126; `SortField` with 6 variants at line 134; `is_travel_time()` helper at line 168 |
| `Cargo.toml` | New deps: wasm-bindgen-futures, gloo-net | VERIFIED | Line 20: `wasm-bindgen-futures = "0.4"`, Line 21: `gloo-net = { version = "0.6", features = ["json"] }` |
| `src/components/address_input.rs` | AddressInput component with debounced geocoding | VERIFIED | 153 lines; `use_debounce_fn_with_arg` at 500ms; suggestion dropdown; clear button; German labels |
| `src/components/school_card.rs` | Extended card with travel time row | VERIFIED | Lines 33-37: accepts optional `travel_times` and `travel_loading` props; Lines 91-140: conditional travel time rendering |
| `src/components/sort_controls.rs` | Extended sort with travel time options | VERIFIED | Lines 10: `has_travel_times` optional prop; Lines 31-42: conditional "Fahrzeit (zu Fuss/Fahrrad/Auto)" options |
| `src/pages/listing.rs` | Full wiring: address input, travel signals, sort, URL params | VERIFIED | 593 lines; address_coords parsed from URL `from=` param; `travel_times` RwSignal; Effect triggers `fetch_all_travel_times`; Memo reads `travel_times.get()` per Pitfall 5 |
| `public/styles.css` | Styles for address input, suggestions, travel time rows | VERIFIED | Lines 679-828: `.address-input-container`, `.address-suggestions`, `.card-travel-times`, `.travel-error`, mobile responsive |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `src/services/geocoding.rs` | photon.komoot.io | `gloo_net::http::Request::get` | WIRED | Line 65: URL template with Berlin bias; Line 68: `Request::get(&url).send()` |
| `src/services/routing.rs` | valhalla1.openstreetmap.de | `gloo_net::http::Request::post` + GET fallback | WIRED | Lines 39-61: POST first, match on response, GET fallback with `?json=` encoding |
| `src/models.rs` -> `src/services/routing.rs` | TravelTimes struct as return type | import + usage | WIRED | routing.rs line 5: `use crate::models::TravelTimes`; line 116: constructs TravelTimes in loop |
| `src/components/address_input.rs` -> `src/services/geocoding.rs` | calls geocode_address | spawn_local | WIRED | Line 5: imports `geocode_address`; Line 30: `geocode_address(&query).await` inside spawn_local |
| `src/pages/listing.rs` -> `src/services/routing.rs` | calls fetch_all_travel_times | spawn_local in Effect | WIRED | Line 16: import; Line 488: `fetch_all_travel_times(lat, lng, school_coords).await` |
| `src/pages/listing.rs` -> URL query params | `from=lat,lng` | build_query_string | WIRED | Line 76: serializes `from={:.6},{:.6}`; Line 257: parses `from` param; all callbacks pass `address_coords.get()` |
| `src/components/school_card.rs` -> `src/models.rs` | receives TravelTimes prop | Signal<Option<TravelTimes>> | WIRED | Line 3: `use crate::models::TravelTimes`; Line 35: `travel_times: Option<Signal<Option<TravelTimes>>>` |
| `src/pages/listing.rs` -> `src/components/school_card.rs` | passes travel data | prop binding | WIRED | Lines 582-586: creates per-school `Signal::derive` for travel_times, passes as prop |
| `src/pages/listing.rs` -> `src/components/sort_controls.rs` | passes has_travel_times | Signal<bool> | WIRED | Line 508: `has_travel_times = Signal::derive(...)`, Line 544: passed as prop |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `address_input.rs` | `suggestions: RwSignal<Vec<PhotonFeature>>` | `geocode_address()` via spawn_local | Live API call to photon.komoot.io | FLOWING (runtime-dependent) |
| `listing.rs` | `travel_times: RwSignal<Option<HashMap<...>>>` | `fetch_all_travel_times()` via Effect + spawn_local | Live API call to valhalla1.openstreetmap.de | FLOWING (runtime-dependent) |
| `school_card.rs` | `travel_times: Option<Signal<Option<TravelTimes>>>` | Per-school Signal::derive from listing.rs travel_times HashMap | Derived from listing.rs signal | FLOWING (prop passed at line 586) |
| `sort_controls.rs` | `has_travel_times: Option<Signal<bool>>` | `travel_times.get().is_some()` | Derived from listing.rs signal | FLOWING (prop passed at line 544) |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Project compiles | `cargo check` | Finished with 3 warnings (unused fields, unused method -- expected) | PASS |
| Commit hashes valid | `git log --oneline` on 4 hashes | All 4 commits (a2abb3c, 208be98, 80ffc17, 30cb641) verified | PASS |
| No stub patterns | grep for TODO/FIXME/PLACEHOLDER | No matches (only HTML placeholder attribute) | PASS |
| No empty implementations | grep for `return null/return {}/=> {}` | No stub patterns found | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| TRVL-01 | 04-01, 04-02 | User can enter their address to calculate travel time to each school | SATISFIED | AddressInput component with Photon geocoding wired to listing page; address_coords parsed from URL `from=` param triggers Effect that calls `fetch_all_travel_times` |
| TRVL-02 | 04-01, 04-02 | Travel time shown for walking, public transport, and car | SATISFIED (with documented adaptation) | Walk/bike/car shown instead of walk/transit/car; bicycle replaces transit because Valhalla matrix API does not support multimodal costing (documented in 04-RESEARCH.md Pitfall 1). SchoolCard shows all three modes with emoji icons |
| TRVL-03 | 04-02 | User can sort/filter schools by travel time from their address | SATISFIED | SortField has TravelTimeWalk/TravelTimeBike/TravelTimeCar variants; filter_and_sort handles all three with None-sorts-last; SortControls conditionally shows travel time sort options |

**Note on TRVL-02:** The requirement says "public transport" but the implementation uses "bicycle" instead. This is a researched adaptation documented in 04-RESEARCH.md: the Valhalla matrix API (`/sources_to_targets`) does not support `multimodal` (transit) costing. The ROADMAP.md success criteria were updated to say "walking, cycling, and driving" reflecting this adaptation. REQUIREMENTS.md still says "public transport" but was marked complete. This is a documented scope adjustment, not a gap.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `src/models.rs` | 168 | `is_travel_time()` method unused (compiler warning) | Info | Method exists for future use; no functional impact |
| `src/services/geocoding.rs` | 26-27 | `state` and `country` fields never read (compiler warning) | Info | Deserialization fields for completeness; no functional impact |
| `src/services/routing.rs` | 15-16 | `distance` and `to_index` fields never read (compiler warning) | Info | Deserialization fields for completeness; no functional impact |

No blocker or warning-level anti-patterns found.

### Human Verification Required

### 1. Address Input Geocoding (Live API)

**Test:** Run `trunk serve`, open browser, type "Alexanderplatz" into the address input
**Expected:** After 500ms debounce, dropdown with up to 5 Berlin-area suggestions appears from Photon API
**Why human:** Photon API CORS behavior (Pitfall 2) and response format can only be verified at runtime in a browser

### 2. Travel Time Display (Live API)

**Test:** Select a geocoded suggestion and observe school cards
**Expected:** Loading state "Berechne Fahrzeit..." appears, then walk/bike/car times in minutes on each card
**Why human:** Valhalla FOSSGIS POST+GET CORS fallback (Pitfall 3) can only be tested in browser with live network

### 3. Travel Time Sorting

**Test:** With travel times loaded, select "Fahrzeit (zu Fuss)" from sort dropdown
**Expected:** Schools reorder ascending by walking time; schools without times sort to bottom
**Why human:** Requires populated travel time data from live API to verify sort correctness

### 4. URL Persistence and Shareability

**Test:** After selecting address, copy URL with `?from=lat,lng`; open in new tab
**Expected:** Travel times auto-load in new tab from URL coordinates
**Why human:** Browser URL routing and navigation state persistence

### 5. Clear Address Flow

**Test:** Click "Adresse loeschen" button
**Expected:** Travel time rows disappear from all cards; URL `from=` param removed; sort options hide travel time entries
**Why human:** Interactive UI state transition across multiple components

### 6. Mobile Responsiveness

**Test:** Resize browser to phone width (~375px)
**Expected:** Address input scales to full width; suggestion dropdown usable; travel time rows wrap correctly
**Why human:** Visual layout verification

### Gaps Summary

No automated gaps found. All 7 observable truths are verified at the code level. All artifacts exist, are substantive (not stubs), and are properly wired. All key links are connected. All 3 requirements (TRVL-01, TRVL-02, TRVL-03) are satisfied with a documented adaptation (bicycle instead of transit).

The only remaining verification is runtime behavior with live external APIs (Photon geocoding, Valhalla routing), which requires human testing in a browser.

---

_Verified: 2026-03-26T20:29:27Z_
_Verifier: Claude (gsd-verifier)_
