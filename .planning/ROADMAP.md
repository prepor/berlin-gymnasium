# Roadmap: Berlin Gymnasien

## Overview

Two-milestone project. Milestone 1 (M1) builds a reproducible Python scraping pipeline that produces a validated YAML corpus of all ~90 Berlin Gymnasien — this is the data contract everything else depends on. Milestone 2 (M2) builds a Rust/Leptos WASM SPA deployed to GitHub Pages that consumes that corpus and gives parents a fast, filterable directory with an interactive map, travel time routing, and side-by-side school comparison.

## Milestones

- **M1: Data Pipeline** — Phases 1 (complete, reproducible YAML corpus)
- **M2: Website SPA** — Phases 2–5 (filterable directory → map → travel time → comparison)

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Data Pipeline** - Reproducible Python pipeline producing a validated YAML corpus of all Berlin Gymnasien
- [ ] **Phase 2: SPA Foundation and School Directory** - Leptos WASM SPA deployed to GitHub Pages with filterable school listing and detail pages
- [x] **Phase 3: Interactive Map** - OpenStreetMap map with clickable pins synced to active filters (completed 2026-03-26)
- [ ] **Phase 4: Travel Time Routing** - User address input with travel time matrix (walk, bike, car) and sort-by-commute
- [ ] **Phase 5: Comparison, Favorites, and Sharing** - Side-by-side comparison, favorites shortlist, and shareable permalinks

## Phase Details

### Phase 1: Data Pipeline
**Goal**: A complete, validated, reproducible YAML corpus of all Berlin Gymnasien exists and can be re-run to produce a field-level changelog
**Depends on**: Nothing (first phase)
**Requirements**: DATA-01, DATA-02, DATA-03, DATA-04, DATA-05, DATA-06, DATA-07, DATA-08, DATA-09, DATA-10, DATA-11, DATA-12
**Success Criteria** (what must be TRUE):
  1. Running the pipeline with a single command produces one YAML file per school in `data/schools/` for all ~90 Berlin Gymnasien
  2. Each school YAML file contains coordinates, district, grundständig flag, contact info, profile/specialization, languages, Ganztag status, open day dates, ratings (with source attribution), and admission requirements where available
  3. All YAML files pass Pydantic schema validation with no errors before being written
  4. Re-running the pipeline produces `data/CHANGELOG.md` with field-level diffs showing exactly what changed
  5. Pipeline setup and execution are documented so a new developer can reproduce the corpus from scratch
**Plans**: 5 plans

Plans:
- [x] 01-01-PLAN.md — Python scaffold (pyproject.toml, justfile, pipeline.yaml) + Pydantic SchoolRecord schema
- [x] 01-02-PLAN.md — Seed step: WFS fetch + Eckdaten XLSX merge → schools_index.yaml
- [x] 01-03-PLAN.md — Agent enrichment: Claude claude-sonnet-4-6 batches with web_search_20260209
- [x] 01-04-PLAN.md — Validate + merge (D-17 structured wins) + YAML writer + changelog
- [x] 01-05-PLAN.md — Orchestrator (run.py wiring all steps) + README + end-to-end verification

### Phase 2: SPA Foundation and School Directory
**Goal**: Parents can browse, filter, and read detailed information about all Berlin Gymnasien on a live GitHub Pages site
**Depends on**: Phase 1
**Requirements**: LIST-01, LIST-02, LIST-03, LIST-04, LIST-05, LIST-06, LIST-07, DETL-01, DETL-02, DETL-03, DETL-04, DETL-05, DETL-06, DEPL-01, DEPL-02, DEPL-03
**Success Criteria** (what must be TRUE):
  1. A user can open the GitHub Pages URL and see a listing of all Berlin Gymnasien with name, district, profile, and grundständig flag
  2. A user can filter schools by district, profile, grundständig status, language offered, and Ganztag mode — combinations work correctly
  3. A user can click any school to view a detail page showing all available fields (contact info, ratings by source, admission requirements, open day date, data freshness)
  4. The site loads and is fully usable on a mobile phone screen
  5. Deploying a new version is triggered by pushing to main (Trunk build → GitHub Pages)
**Plans**: 3 plans

Plans:
- [x] 02-01-PLAN.md — Rust project scaffold + build.rs data embedding + app shell with router + GitHub Actions deploy workflow
- [x] 02-02-PLAN.md — School listing page with card grid, filter panel, sort controls, and URL-persisted filter state
- [x] 02-03-PLAN.md — School detail page with all data sections, ratings, admission, contact, and "Keine Angabe" handling
**UI hint**: yes

### Phase 3: Interactive Map
**Goal**: Parents can see all (filtered) schools on an interactive map and navigate to school detail pages from map pins
**Depends on**: Phase 2
**Requirements**: MAP-01, MAP-02, MAP-03
**Success Criteria** (what must be TRUE):
  1. A user can switch to a map view and see pins for all schools at their correct coordinates on an OpenStreetMap base layer
  2. A user can click any pin to see the school name and navigate to its detail page
  3. Applying filters in the listing view removes the corresponding pins from the map in real time
**Plans**: 1 plan

Plans:
- [x] 03-01-PLAN.md — Leaflet.js map integration via wasm-bindgen with color-coded pins, popups, filter sync, and list/map view toggle
**UI hint**: yes

### Phase 4: Travel Time Routing
**Goal**: Parents can enter their home address and see how long it takes to reach each school by walking, cycling, and driving — and sort schools by commute time
**Depends on**: Phase 3
**Requirements**: TRVL-01, TRVL-02, TRVL-03
**Success Criteria** (what must be TRUE):
  1. A user can type their home address into an input field and have it geocoded to coordinates via Photon/komoot
  2. After entering an address, each school in the listing shows travel time for walking, cycling, and driving
  3. A user can sort the school listing by any travel mode's commute time from their entered address
**Plans**: 2 plans

Plans:
- [x] 04-01-PLAN.md — Data model (TravelTimes, SortField extension), Cargo deps (gloo-net, wasm-bindgen-futures), Photon geocoding + Valhalla routing service modules
- [ ] 04-02-PLAN.md — AddressInput component, listing page integration (URL params, reactive signals, travel time fetch), SchoolCard travel row, sort controls extension, CSS
**UI hint**: yes

### Phase 5: Comparison, Favorites, and Sharing
**Goal**: Parents can compare 2–3 schools side by side, save a favorites shortlist, and share school links
**Depends on**: Phase 4
**Requirements**: COMP-01, COMP-02, COMP-03
**Success Criteria** (what must be TRUE):
  1. A user can select 2–3 schools and view them in a side-by-side comparison layout showing all key fields
  2. Each school has a shareable permalink URL that opens directly to its detail page
  3. A user can save schools to a favorites shortlist that persists across browser sessions (localStorage) and can be reviewed at any time
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Data Pipeline | 5/5 | Complete | 2026-03-26 |
| 2. SPA Foundation and School Directory | 3/3 | Complete | 2026-03-26 |
| 3. Interactive Map | 1/1 | Complete   | 2026-03-26 |
| 4. Travel Time Routing | 0/2 | Planned | - |
| 5. Comparison, Favorites, and Sharing | 0/TBD | Not started | - |
