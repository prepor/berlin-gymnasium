# Berlin Gymnasien

## What This Is

A comparison website for all Berlin Gymnasien (106 schools), helping parents choose the right secondary school. Two parts: a reproducible Python data pipeline (Claude AI agents + Berlin Open Data) and a Rust/Leptos WASM SPA deployed to GitHub Pages with filtering, interactive map, and travel time routing.

## Core Value

Parents can quickly find and compare Berlin Gymnasien based on the criteria that matter most — ratings, location, specialization, and whether the school accepts students after 4th grade.

## Current State

**Shipped:** v1.0 MVP (2026-03-26)

- 106 Berlin Gymnasien scraped and validated as YAML files
- Filterable school directory with card grid, 5 filter types, URL-persisted state
- School detail pages with ratings, admission requirements, contact info
- Interactive OpenStreetMap with color-coded pins synced to filters
- Travel time routing (walk/bike/car) via Photon geocoding + Valhalla matrix
- Deployed to GitHub Pages via Trunk + GitHub Actions

**Tech stack:** Rust/Leptos 0.8 (WASM CSR), Leaflet.js (via wasm-bindgen), Python 3.12+ pipeline with Anthropic SDK

## Requirements

### Validated

- Scrape and validate data for all Berlin Gymnasien — v1.0
- One YAML file per school with structured data — v1.0
- Collect ratings from discoverable sources — v1.0
- Flag which schools accept students after 4th grade — v1.0
- Reproducible scraping pipeline with changelog on re-run — v1.0
- Document how data was gathered — v1.0
- Static SPA with school listing and filters — v1.0
- Interactive map showing school locations — v1.0
- Travel time estimation (walk, bike, car) from user-entered address — v1.0
- Deploy to GitHub Pages — v1.0

### Active

(None — next milestone not yet planned)

### Out of Scope

- Dynamic backend / database — static site only
- User accounts / login — no server-side state
- Real-time data updates — manual re-run of pipeline
- Mobile native app — responsive web only
- Google Maps rating scraping — violates ToS

## Constraints

- **Tech stack (site)**: Rust + Leptos — static SPA compiled to WASM
- **Deployment**: GitHub Pages — no server-side rendering or API
- **Data format**: YAML files, one per school, in `data/schools/`
- **Travel time API**: Valhalla FOSSGIS (free, CORS-enabled) + Photon/komoot for geocoding
- **Scraping**: Reproducible pipeline with field-level changelog

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + Leptos for frontend | User preference, WASM-based SPA | ✓ Shipped v1.0 |
| One YAML file per school | Clean organization, easy to diff/review | ✓ 106 files |
| GitHub Pages deployment | Free, simple, no backend needed | ✓ Working |
| Valhalla FOSSGIS for travel time | Free, CORS-enabled, supports walk/bike/car matrix | ✓ Working |
| Photon/komoot for geocoding | CORS-enabled, no API key, browser-safe | ✓ Working |
| Leaflet.js via wasm-bindgen (not leptos-leaflet) | leptos-leaflet incompatible with Leptos 0.8 | ✓ Working |
| Bicycle instead of transit | Valhalla matrix API doesn't support multimodal costing | ✓ Documented adaptation |
| build.rs YAML→JSON embedding | Compile-time data, no runtime fetch needed | ✓ 106 schools embedded |
| Claude AI agents for enrichment | Web research per school via batched tool use | ✓ Pipeline working |

---
*Last updated: 2026-03-26 after v1.0 milestone*
