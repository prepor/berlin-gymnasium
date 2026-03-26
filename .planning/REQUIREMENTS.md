# Requirements: Berlin Gymnasien

**Defined:** 2026-03-26
**Core Value:** Parents can quickly find and compare Berlin Gymnasien based on the criteria that matter most — ratings, location, specialization, and whether the school accepts students after 4th grade.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Data Pipeline

- [x] **DATA-01**: Pipeline seeds canonical list of all Berlin Gymnasien from Berlin Open Data WFS with school IDs, coordinates, and district
- [x] **DATA-02**: Pipeline scrapes structured data (address, phone, website, student count, teacher count) from official sources
- [x] **DATA-03**: Claude AI agents enrich each school with profile/specialization, languages offered, Ganztag status, open day dates, and ratings from discoverable sources
- [x] **DATA-04**: Each school stored as one YAML file in `data/schools/{school_id}.yaml` with defined schema
- [x] **DATA-05**: YAML schema includes provenance tracking (data_sources, last_updated per school)
- [x] **DATA-06**: Pipeline re-run produces field-level changelog (`data/CHANGELOG.md`) showing what changed
- [x] **DATA-07**: Pipeline runs reproducibly with documented setup and single command execution
- [x] **DATA-08**: Pipeline validates all school records against Pydantic schema before writing
- [x] **DATA-09**: Pipeline collects ratings from multiple permitted sources (not Google Maps scraping) with source attribution, scale, and review count
- [x] **DATA-10**: Pipeline flags which schools accept students after 4th grade (grundständig)
- [x] **DATA-11**: Pipeline collects admission requirements per school (Notendurchschnitt, selection criteria, oversubscription info) where publicly available
- [x] **DATA-12**: Pipeline collects open day (Tag der offenen Tür) dates from school websites

### School Listing & Filters

- [ ] **LIST-01**: User can view a listing of all Berlin Gymnasien with name, district, profile, and grundständig flag
- [ ] **LIST-02**: User can filter schools by district (Bezirk)
- [ ] **LIST-03**: User can filter schools by profile/specialization (MINT, music, sports, bilingual, altsprachlich)
- [ ] **LIST-04**: User can filter schools by grundständig status (accepts after 4th grade)
- [ ] **LIST-05**: User can filter schools by foreign language offered
- [ ] **LIST-06**: User can filter schools by Ganztag (full-day) vs half-day
- [ ] **LIST-07**: Listing is mobile-responsive and usable on phone screens

### School Detail

- [x] **DETL-01**: User can view a detail page for each school showing all available data fields
- [x] **DETL-02**: Detail page shows contact info (phone, email, website link)
- [x] **DETL-03**: Detail page shows ratings from each source with source name, score, scale, and review count
- [x] **DETL-04**: Detail page shows admission requirements where available
- [x] **DETL-05**: Detail page shows next open day date where available
- [x] **DETL-06**: Detail page shows data freshness indicator (last updated date)

### Map

- [ ] **MAP-01**: User can view all schools on an interactive OpenStreetMap-based map
- [ ] **MAP-02**: User can click a map pin to see school name and navigate to detail page
- [ ] **MAP-03**: Map reflects active filters (only shows filtered schools)

### Travel Time

- [ ] **TRVL-01**: User can enter their address to calculate travel time to each school
- [ ] **TRVL-02**: Travel time shown for walking, public transport, and car
- [ ] **TRVL-03**: User can sort/filter schools by travel time from their address

### Comparison & Sharing

- [ ] **COMP-01**: User can select 2-3 schools for side-by-side comparison view
- [ ] **COMP-02**: Each school has a shareable permalink URL
- [ ] **COMP-03**: User can save schools to a favorites shortlist (persisted in localStorage)

### Deployment

- [x] **DEPL-01**: Site is a static Rust/Leptos WASM SPA compiled with Trunk
- [x] **DEPL-02**: Site is deployed to GitHub Pages
- [x] **DEPL-03**: School data embedded at build time (YAML → JSON → include_str!)

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Enhanced Data

- **EDAT-01**: Aggregated normalized rating score across multiple sources
- **EDAT-02**: Historical data tracking (changes over time visualized)
- **EDAT-03**: School facilities details (cafeteria quality, lab equipment, library)

### Enhanced UI

- **EUI-01**: "What's new" changelog surfaced in the UI for returning visitors
- **EUI-02**: Open day calendar view across all schools
- **EUI-03**: RSS/Atom feed of data changes

### Automation

- **AUTO-01**: GitHub Actions scheduled pipeline re-run (monthly)
- **AUTO-02**: Automated PR creation with changelog for human review

## Out of Scope

| Feature | Reason |
|---------|--------|
| User accounts / login | No backend; static site only; GDPR surface area |
| User reviews / comments | Moderation burden, legal exposure; link to existing platforms instead |
| School rankings / league tables | Tagesspiegel already does this; controversial; not the product's goal |
| Recommendation engine / AI match | Scope creep; filters + comparison serves same need |
| Catchment area polygons | Berlin has open choice (no catchment); travel time is the right model |
| Real-time data | No backend possible on GitHub Pages; pipeline re-run is sufficient |
| Mobile native app | Responsive web is sufficient for v1 |
| Google Maps rating scraping | Violates Google ToS; use only permitted rating sources |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DATA-01 | Phase 1 | Complete |
| DATA-02 | Phase 1 | Complete |
| DATA-03 | Phase 1 | Complete |
| DATA-04 | Phase 1 | Complete |
| DATA-05 | Phase 1 | Complete |
| DATA-06 | Phase 1 | Complete |
| DATA-07 | Phase 1 | Complete |
| DATA-08 | Phase 1 | Complete |
| DATA-09 | Phase 1 | Complete |
| DATA-10 | Phase 1 | Complete |
| DATA-11 | Phase 1 | Complete |
| DATA-12 | Phase 1 | Complete |
| LIST-01 | Phase 2 | Pending |
| LIST-02 | Phase 2 | Pending |
| LIST-03 | Phase 2 | Pending |
| LIST-04 | Phase 2 | Pending |
| LIST-05 | Phase 2 | Pending |
| LIST-06 | Phase 2 | Pending |
| LIST-07 | Phase 2 | Pending |
| DETL-01 | Phase 2 | Complete |
| DETL-02 | Phase 2 | Complete |
| DETL-03 | Phase 2 | Complete |
| DETL-04 | Phase 2 | Complete |
| DETL-05 | Phase 2 | Complete |
| DETL-06 | Phase 2 | Complete |
| MAP-01 | Phase 3 | Pending |
| MAP-02 | Phase 3 | Pending |
| MAP-03 | Phase 3 | Pending |
| TRVL-01 | Phase 4 | Pending |
| TRVL-02 | Phase 4 | Pending |
| TRVL-03 | Phase 4 | Pending |
| COMP-01 | Phase 5 | Pending |
| COMP-02 | Phase 5 | Pending |
| COMP-03 | Phase 5 | Pending |
| DEPL-01 | Phase 2 | Complete |
| DEPL-02 | Phase 2 | Complete |
| DEPL-03 | Phase 2 | Complete |

**Coverage:**
- v1 requirements: 36 total
- Mapped to phases: 36
- Unmapped: 0

---
*Requirements defined: 2026-03-26*
*Last updated: 2026-03-26 after roadmap creation*
