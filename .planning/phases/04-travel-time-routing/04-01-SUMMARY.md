---
phase: 04-travel-time-routing
plan: 01
subsystem: api
tags: [valhalla, photon, geocoding, routing, wasm, gloo-net, travel-time]

# Dependency graph
requires:
  - phase: 02-frontend-core
    provides: models.rs with School/Coords/SortField, Cargo.toml with base deps
provides:
  - TravelTimes struct for per-school travel time data
  - SortField travel time variants (walk/bike/car)
  - Photon geocoding service (geocode_address async fn)
  - Valhalla matrix routing service (fetch_travel_times_matrix, fetch_all_travel_times)
  - is_travel_time() helper on SortField
affects: [04-02-travel-time-ui]

# Tech tracking
tech-stack:
  added: [wasm-bindgen-futures 0.4, gloo-net 0.6]
  patterns: [async HTTP from WASM via gloo-net, POST+GET CORS fallback for Valhalla, Berlin-biased Photon geocoding]

key-files:
  created:
    - src/services/mod.rs
    - src/services/geocoding.rs
    - src/services/routing.rs
  modified:
    - Cargo.toml
    - src/models.rs
    - src/main.rs
    - src/pages/listing.rs

key-decisions:
  - "Use gloo-net for WASM HTTP (ergonomic, lightweight) over raw web-sys fetch"
  - "Bicycle instead of transit for third mode (Valhalla matrix does not support multimodal)"
  - "POST+GET fallback for Valhalla CORS (per Pitfall 3)"
  - "Travel time sort falls back to name sort until wired in Plan 04-02"

patterns-established:
  - "Async service modules in src/services/ for external API calls"
  - "POST-first with GET fallback pattern for CORS-uncertain APIs"
  - "GeoJSON [lng,lat] coordinate extraction helpers on response types"

requirements-completed: [TRVL-01, TRVL-02]

# Metrics
duration: 3min
completed: 2026-03-26
---

# Phase 4 Plan 1: Travel Time Services Summary

**Photon geocoding + Valhalla matrix routing services with TravelTimes model and SortField travel variants using gloo-net async HTTP**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-26T20:14:14Z
- **Completed:** 2026-03-26T20:17:16Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Added wasm-bindgen-futures and gloo-net dependencies for async HTTP from WASM
- Created TravelTimes struct and extended SortField with walk/bike/car travel time sort variants
- Built Photon geocoding service with Berlin-biased search (lat=52.52, lon=13.405, limit=5, lang=de)
- Built Valhalla matrix routing service with POST+GET CORS fallback and seconds-to-minutes conversion
- Created fetch_all_travel_times convenience function mapping school IDs to TravelTimes via index tracking

## Task Commits

Each task was committed atomically:

1. **Task 1: Add dependencies and create TravelTimes model + extended SortField** - `a2abb3c` (feat)
2. **Task 2: Create geocoding and routing service modules** - `208be98` (feat)

## Files Created/Modified
- `Cargo.toml` - Added wasm-bindgen-futures and gloo-net dependencies
- `src/models.rs` - Added TravelTimes struct, extended SortField with 3 travel variants, added is_travel_time() helper
- `src/main.rs` - Added mod services declaration
- `src/services/mod.rs` - Module declarations for geocoding and routing
- `src/services/geocoding.rs` - Photon API integration with PhotonFeature types and geocode_address async fn
- `src/services/routing.rs` - Valhalla matrix API with POST+GET fallback, fetch_travel_times_matrix and fetch_all_travel_times
- `src/pages/listing.rs` - Added placeholder match arms for travel time sort variants

## Decisions Made
- Used gloo-net for WASM HTTP requests (ergonomic, lightweight, standard for WASM fetch)
- Chose bicycle as third mode instead of transit (Valhalla matrix endpoint does not support multimodal costing)
- Implemented POST+GET fallback pattern for Valhalla (POST with JSON body first, GET with ?json= query param if CORS preflight fails)
- Travel time sort variants fall back to name sorting until wired with actual data in Plan 04-02

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added exhaustive match arms for travel time sort in listing.rs**
- **Found during:** Task 1 (SortField enum extension)
- **Issue:** Adding new SortField variants caused non-exhaustive match error in src/pages/listing.rs filter_and_sort function
- **Fix:** Added match arms for TravelTimeWalk/Bike/Car that fall back to name sorting (actual travel time sort will be wired in Plan 04-02)
- **Files modified:** src/pages/listing.rs
- **Verification:** cargo check passes
- **Committed in:** a2abb3c (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary to maintain compilation. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All service functions compile and are ready to be wired into UI components in Plan 04-02
- AddressInput component, school card travel time display, and sort integration are the next steps
- Photon CORS and Valhalla CORS will be tested at runtime when the UI is built

## Self-Check: PASSED
- All 7 files verified present on disk
- Both commit hashes (a2abb3c, 208be98) verified in git log
