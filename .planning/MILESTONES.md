# Milestones

## v1.0 MVP (Shipped: 2026-03-26)

**Phases completed:** 4 phases, 11 plans, 22 tasks

**Key accomplishments:**

- uv-managed Python pipeline scaffold with Pydantic SchoolRecord schema (8 fields for DATA-04/09/10/11/12), justfile task runner, and pipeline.yaml config — schema contract for all downstream pipeline modules
- Async WFS fetch from gdi.berlin.de/services/wfs/schulen with EPSG:4326, merged with Eckdaten XLSX student/teacher counts via openpyxl, writes data/schools_index.yaml
- Batched Claude Sonnet agents with web_search_20260209 tool research schools for profiles, languages, ratings, admission requirements, open days, images, and social media
- Merge-and-validate module (D-17 structured-wins + Pydantic) and YAML writer with deepdiff changelog and conflicts.md regeneration
- pipeline/run.py orchestrator wiring seed → enrich → validate → write with ANTHROPIC_API_KEY fail-fast, and README.md covering full setup and usage documentation
- Rust/Leptos WASM SPA scaffolded with build-time embedding of 106 school YAML files, two-route app shell, and GitHub Actions deployment to Pages
- Filterable card grid with five filter types (district, profile, grundstaendig, language, ganztag), URL-persisted filter state, sort controls, and responsive mobile-first CSS
- Full school detail page with 7 sections (hero, profile/languages, admission, ratings, open day, contact, data provenance), Keine Angabe fallback for missing data, and responsive CSS
- Leaflet.js map with color-coded CircleMarker pins, profile popups, filter-synced markers, and list/map view toggle via URL query param
- Photon geocoding + Valhalla matrix routing services with TravelTimes model and SortField travel variants using gloo-net async HTTP
- AddressInput component with debounced Photon geocoding, travel time display on school cards (walk/bike/car), sort-by-commute options, URL param persistence, and full CSS

---
