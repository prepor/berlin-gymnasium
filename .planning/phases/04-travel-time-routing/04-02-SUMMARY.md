---
phase: 04-travel-time-routing
plan: 02
subsystem: ui
tags: [address-input, travel-time, geocoding, sort, leptos, css, url-params]

# Dependency graph
requires:
  - phase: 04-travel-time-routing
    plan: 01
    provides: TravelTimes struct, SortField travel variants, geocoding/routing services
provides:
  - AddressInput component with debounced Photon geocoding and suggestion dropdown
  - SchoolCard travel time row (walk/bike/car with loading and error states)
  - SortControls travel time sort options (conditionally shown)
  - URL ?from=lat,lng param for address persistence and shareability
  - Full CSS for address input, suggestions, travel time cards, error states
affects: []

# Tech tracking
tech-stack:
  added: [leptos-use debounce]
  patterns: [debounced input with spawn_local geocoding, Effect-driven travel time fetch, travel_times RwSignal read inside Memo for reactive sorting]

key-files:
  created:
    - src/components/address_input.rs
  modified:
    - src/components/mod.rs
    - src/components/school_card.rs
    - src/components/sort_controls.rs
    - src/pages/listing.rs
    - public/styles.css

key-decisions:
  - "AddressInput uses leptos-use use_debounce_fn_with_arg (500ms) for geocoding to avoid excessive API calls"
  - "Travel times stored as RwSignal<Option<HashMap>> separate from School data, read inside Memo for reactive sort dependency"
  - "All filter/sort/view callbacks pass address_coords through navigation to preserve from= param"
  - "SchoolCard/SortControls use #[prop(optional)] for backward-compatible travel time props"

patterns-established:
  - "Debounced input with spawn_local async pattern for API-driven suggestions"
  - "Effect watching URL-derived signal to trigger async fetch (address_coords -> travel_times)"
  - "Optional props with Signal<Option<T>> for progressively enhanced components"

requirements-completed: [TRVL-01, TRVL-02, TRVL-03]

# Metrics
duration: 5min
completed: 2026-03-26
---

# Phase 4 Plan 2: Travel Time UI Summary

**AddressInput component with debounced Photon geocoding, travel time display on school cards (walk/bike/car), sort-by-commute options, URL param persistence, and full CSS**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-26T20:19:39Z
- **Completed:** 2026-03-26T20:25:12Z
- **Tasks:** 3 (2 auto + 1 auto-approved checkpoint)
- **Files modified:** 6

## Accomplishments
- Created AddressInput component with 500ms debounced Photon geocoding, suggestion dropdown, clear button, German labels
- Extended ListingPage with address coords URL param (?from=lat,lng), travel time RwSignal, Effect-driven fetch, and reactive Memo sorting
- Extended SchoolCard with conditional travel time row: walk/bike/car emoji display, loading state, no-data state, em-dash for unreachable routes
- Extended SortControls with conditionally-shown travel time sort options (zu Fuss, Fahrrad, Auto)
- Added comprehensive CSS for address input, suggestion overlay, travel time card rows, error banner, and mobile responsiveness
- Travel time sorting correctly handles None values (schools without times sort last)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create AddressInput and wire travel time signals into ListingPage** - `80ffc17` (feat)
2. **Task 2: Add travel time CSS for address input, cards, and error states** - `30cb641` (feat)
3. **Task 3: Verify travel time feature end-to-end** - Auto-approved checkpoint (no commit)

## Files Created/Modified
- `src/components/address_input.rs` - New AddressInput component with debounced geocoding, suggestion dropdown, clear button
- `src/components/mod.rs` - Added address_input module declaration
- `src/components/school_card.rs` - Extended with optional travel_times and travel_loading props, conditional travel time row
- `src/components/sort_controls.rs` - Extended with optional has_travel_times prop, conditional travel time sort options
- `src/pages/listing.rs` - Major integration: address URL param, travel time signals, Effect fetch, Memo with travel_times dependency, address callbacks
- `public/styles.css` - Added 151 lines of travel-time-related CSS (address input, suggestions, card rows, error, mobile)

## Decisions Made
- Used `use_debounce_fn_with_arg` from leptos-use for geocoding debounce (500ms, per D-19)
- Stored travel times as `RwSignal<Option<HashMap<String, TravelTimes>>>` separate from school data (ephemeral per-session)
- Read `travel_times.get()` inside the `filtered_schools` Memo closure to register reactive dependency (per Pitfall 5)
- All existing filter/sort/view callbacks pass `address_coords.get()` through to preserve address across interactions
- SchoolCard and SortControls use `#[prop(optional)]` for backward-compatible optional props
- Removed `ref` pattern from `if let Some(ref tt)` to conform with Rust 2024 edition

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed explicit `ref` in match patterns for Rust 2024 edition**
- **Found during:** Task 1
- **Issue:** Rust 2024 edition does not allow explicit `ref` in implicitly-borrowing patterns (`if let Some(ref tt) = travel_times`)
- **Fix:** Changed to `if let Some(tt) = travel_times` (ref is implicit in 2024 edition)
- **Files modified:** src/pages/listing.rs
- **Verification:** cargo check passes

**2. [Rule 3 - Blocking] SchoolCard and SortControls updated in Task 1 instead of Task 2**
- **Found during:** Task 1
- **Issue:** listing.rs passes travel_times and has_travel_times props to SchoolCard and SortControls; compilation requires these components to accept the props
- **Fix:** Implemented full SchoolCard travel time rendering and SortControls travel time options in Task 1 to avoid compilation failure
- **Impact:** Task 2 only needed CSS addition (no Rust changes), keeping Task 2 focused and clean

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** No scope creep. All acceptance criteria met.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all components are fully wired with real data sources and signals.

## Self-Check: PASSED
- All 6 files verified present on disk
- Both commit hashes (80ffc17, 30cb641) verified in git log
