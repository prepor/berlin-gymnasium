---
phase: 02-spa-foundation-and-school-directory
plan: 02
subsystem: ui
tags: [rust, leptos, wasm, filtering, url-state, responsive-css, school-cards]

requires:
  - phase: 02-spa-foundation-and-school-directory
    plan: 01
    provides: "Compilable Leptos CSR project with routing and build-time data embedding"
provides:
  - "Filterable school listing page at / with card grid"
  - "Five filter types: district, profile, grundstaendig, language, ganztag"
  - "URL query param persistence for shareable filtered views"
  - "Sort by name, district, or student count"
  - "Responsive CSS: mobile 1-col, tablet 2-col, desktop 3-col"
  - "SchoolCard, FilterPanel, FilterChips, SortControls reusable components"
affects: [02-spa-foundation-and-school-directory, 03-interactive-map, 05-comparison]

tech-stack:
  added: [leptos-use 0.18]
  patterns: [URL query param filter state via use_query_map/use_navigate, Memo-based derived filtered list, For-based keyed rendering, details/summary for mobile collapse]

key-files:
  created: [src/components/school_card.rs, src/components/filter_panel.rs, src/components/filter_chips.rs, src/components/sort_controls.rs]
  modified: [src/models.rs, src/pages/listing.rs, src/components/mod.rs, Cargo.toml, public/styles.css]

key-decisions:
  - "Filter state stored in URL query params with comma-separated multi-values (not hash params)"
  - "Memo-based derived signal for filtered+sorted school list"
  - "CSS details/summary element for mobile filter collapse (no JS needed)"
  - "Profile badge colors via inline styles per plan specification"

patterns-established:
  - "URL filter state: use_query_map reads, use_navigate with replace:true writes"
  - "Comma-separated multi-value params: ?district=Mitte,Pankow&profile=MINT"
  - "Tri-state boolean filters: ja/nein/absent in URL"
  - "Card component pattern: takes School, renders as clickable <a> block"
  - "Filter callbacks: toggle for multi-select, set for tri-state"

requirements-completed: [LIST-01, LIST-02, LIST-03, LIST-04, LIST-05, LIST-06, LIST-07]

duration: 5min
completed: 2026-03-26
---

# Phase 02 Plan 02: School Listing and Filters Summary

**Filterable card grid with five filter types (district, profile, grundstaendig, language, ganztag), URL-persisted filter state, sort controls, and responsive mobile-first CSS**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-26T18:32:34Z
- **Completed:** 2026-03-26T18:38:28Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- All 106 schools render as clickable cards with name, district, profile badges, grundstaendig flag, student count, and completeness bar
- Five filter types combine with AND logic: district (multi-select), profile (multi-select), grundstaendig (tri-state), language (multi-select), ganztag (tri-state)
- Filter state persists in URL query params so filtered views are shareable and survive page reload
- Sort by Name (A-Z), Bezirk, or Schueleranzahl with URL persistence
- Responsive layout: 1 column on mobile, 2 on tablet, 3 on desktop; filter panel collapses on mobile

## Task Commits

Each task was committed atomically:

1. **Task 1: SchoolCard, FilterPanel, FilterChips, SortControls components** - `ab41cfa` (feat)
2. **Task 2: ListingPage with URL-persisted filters, card grid, and responsive CSS** - `6c4a6dd` (feat)

## Files Created/Modified
- `src/components/school_card.rs` - SchoolCard rendering name, district, profile badges, grundstaendig flag, student count, completeness
- `src/components/filter_panel.rs` - FilterPanel with five filter sections (Bezirk, Profil, Grundstaendig, Fremdsprache, Ganztag)
- `src/components/filter_chips.rs` - Active filter count badge and clear-all button
- `src/components/sort_controls.rs` - Sort dropdown (Name, Bezirk, Schueleranzahl)
- `src/models.rs` - Added SortField enum, all_districts/all_profiles/all_languages helpers
- `src/pages/listing.rs` - Full listing page with URL query param state, filtered Memo, card grid
- `src/components/mod.rs` - Re-exports for new components
- `Cargo.toml` - Added leptos-use dependency
- `public/styles.css` - Complete card grid, filter sidebar, responsive breakpoints, all component styles

## Decisions Made
- Used URL query params with comma-separated values for multi-select filters (shareable URLs)
- Used Memo for derived filtered+sorted school list (efficient reactivity)
- Used CSS details/summary for mobile filter collapse (native HTML, no JS state needed)
- Profile badge colors applied via inline styles as specified in plan (MINT=#22c55e, bilingual=#f97316, etc.)
- Filter panel opens by default with `open=true` on details element; summary only shown on mobile via CSS

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Rust toolchain was not in PATH in this worktree session; resolved by sourcing ~/.cargo/env before each command.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Listing page fully functional with all filters, sort, and responsive layout
- Ready for Plan 03 (detail page enhancement) which depends on the card links (href=/school/{id})
- SchoolCard, FilterPanel, FilterChips, SortControls components are reusable
- trunk build succeeds cleanly

## Self-Check: PASSED

- All 9 created/modified files verified present on disk
- All 2 task commits verified in git log (ab41cfa, 6c4a6dd)
