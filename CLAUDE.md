<!-- GSD:project-start source:PROJECT.md -->
## Project

**Berlin Gymnasien**

A comprehensive comparison website for all Berlin Gymnasien (~90+), helping parents choose the right secondary school for their child. The project has two parts: a reproducible data scraping pipeline that collects and validates school data from multiple sources, and a static SPA built with Rust/Leptos deployed to GitHub Pages.

**Core Value:** Parents can quickly find and compare Berlin Gymnasien based on the criteria that matter most to them — ratings, location, specialization, and whether the school accepts students after 4th grade.

### Constraints

- **Tech stack (site)**: Rust + Leptos — static SPA compiled to WASM
- **Deployment**: GitHub Pages — no server-side rendering or API
- **Data format**: YAML files, one per school, in `data/schools/`
- **Travel time API**: Needs a free/affordable routing API (likely OSRM or similar) — called client-side
- **Scraping**: Must be reproducible and produce a changelog showing what changed between runs
<!-- GSD:project-end -->

<!-- GSD:stack-start source:research/STACK.md -->
## Technology Stack

## Recommended Stack
### Frontend: Rust/Leptos SPA
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| leptos | 0.8.x (0.8.17 latest) | Reactive WASM SPA framework | Project constraint; fine-grained reactivity without virtual DOM diffing; CSR mode compiles to pure WASM SPA |
| leptos_router | bundled with 0.8.x | Client-side routing | Integrated with leptos, supports nested routes, query params for filter state |
| leptos-use | 0.18.x | Utility hooks (debounce, localStorage, URL state) | Equivalent of VueUse/react-use for leptos; use_debounce_fn for address input, use_local_storage for persisting filter state |
| trunk | latest stable | Build tool for CSR/WASM | Official recommended tool for leptos CSR; produces dist/ ready for GitHub Pages |
| wasm-bindgen | bundled via trunk | Rust-to-JS FFI | Required for calling browser APIs and JS libraries from Rust |
### Map Library
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| leptos-leaflet | 0.9.x (latest; 0.7-branch for leptos 0.7, verify 0.8 compat) | Interactive school map | Idiomatic Leptos components wrapping Leaflet.js; provides MapContainer, Marker, TileLayer, Popup, Circle out of the box |
| Leaflet.js | 1.9.x (loaded via CDN or trunk asset) | Underlying map engine pulled by leptos-leaflet | Industry-standard raster tile map; ~40KB; works in all browsers; no WebGL requirement |
| OpenStreetMap tiles | — | Free map tiles | openstreetmap.org tile CDN; free; attribution required; adequate for ~90 markers |
- `leptos-leaflet` is the only production-tested Leptos-native map wrapper with active maintenance.
- MapLibre GL requires WebGL and is heavier; overkill for a 90-marker school locator.
- `kapta` is experimental/low-adoption.
- Leaflet raster tiles are sufficient for this use-case.
### Routing / Travel Time API
| Technology | Type | Purpose | Why |
|------------|------|---------|-----|
| Valhalla (valhalla1.openstreetmap.de) | Free public API | Travel time from user address to school (foot, transit, car) | Supports pedestrian, bicycle, auto, and multimodal transit costing; FOSSGIS-hosted public server; REST/JSON; OSM-based |
| Nominatim (nominatim.openstreetmap.org) | Free public API | Geocode user-entered address to lat/lng | CORS-enabled; no API key; 1 req/sec limit; sufficient for on-demand single geocodes |
- Valhalla supports foot, transit, and car natively in one API. OSRM only supports car/bicycle natively.
- OSRM public server has documented intermittent CORS issues. Valhalla's FOSSGIS server is more reliably CORS-enabled.
- Valhalla's `matrix` and `sources_to_targets` endpoints can compute travel times from one origin to multiple schools in a single call.
### Data Format: Rust YAML Parsing
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| serde | 1.x | Core serialization framework | Standard; used by all Rust data libraries |
| serde-saphyr | 0.0.10 | Deserialize YAML school files in build step | serde_yaml (dtolnay) is unmaintained. serde-saphyr is the fastest maintained replacement, passes full yaml-test-suite |
- Build step: read `data/schools/*.yaml` → parse with serde-saphyr → serialize to a single `schools.json` → embed in WASM or place in `dist/` for fetch.
- WASM runtime: fetch `schools.json`, deserialize with `serde_json`.
### Data Scraping Pipeline
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Python | 3.12+ | Scraping orchestration | Strong ecosystem for HTTP, HTML parsing, and Anthropic SDK |
| anthropic | latest (0.40+) | Claude agents for web research and data extraction | Project constraint; structured outputs mode produces guaranteed-valid schema-conforming JSON |
| httpx | 0.27+ | Async HTTP requests for structured sources | Modern replacement for requests; native async/await; used for concurrent scraping |
| beautifulsoup4 | 4.12+ | HTML parsing from school websites and schulverzeichnis | Industry standard; adequate for parsing static school pages |
| pydantic | 2.x | Schema validation for school YAML data | Validate extracted data before writing; generates JSON schema for Claude prompts |
| ruamel.yaml | 0.18+ | Read/write YAML preserving comments and formatting | Better than PyYAML for round-trip edits and diff-friendly output |
| tenacity | 8.x | Retry logic for API calls | Handles rate limiting, transient errors in Claude and external APIs |
### Berlin Open Data Source (Seed Data)
| Source | Format | What It Provides |
|--------|--------|-----------------|
| daten.berlin.de WFS (Schulen layer) | GeoJSON / WFS | School locations, coordinates, school type codes |
| daten.berlin.de "Eckdaten allgemeinbildende Schulen 2024/2025" | CSV/Excel | Student counts, teacher counts, school numbers (BSN) |
| JedeSchule.de | Web/data download | Structured school profiles, possibly filterable by Gymnasien |
| schulverzeichnis.berlin.de | HTML (scrape) | Admission criteria, languages, contact data |
## Alternatives Considered
| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Map library | leptos-leaflet + Leaflet.js | MapLibre GL | WebGL requirement, heavier, no Leptos-native wrapper, overkill for 90 markers |
| Map library | leptos-leaflet + Leaflet.js | kapta | Experimental, very low adoption, unproven in production |
| Routing API | Valhalla (FOSSGIS) | OSRM (public) | OSRM: car-only natively, documented CORS issues |
| Routing API | Valhalla (FOSSGIS) | OpenRouteService | No CORS on public endpoint; browser calls impossible |
| Routing API | Valhalla (FOSSGIS) | Google Maps Distance Matrix | Paid, requires API key, billing overhead for static site |
| YAML (Rust) | serde-saphyr | serde_yaml | serde_yaml is unmaintained since 2023 |
| YAML (Rust) | serde-saphyr | serde_yml | Lower performance; serde-saphyr is recommended |
| Scraping SDK | anthropic + tool use | Claude Agent SDK | Agent SDK is for interactive sessions; direct API gives pipeline control |
| HTTP client (Python) | httpx | requests | requests lacks native async; httpx supports concurrent fetching |
| Build tool | trunk | cargo-leptos | cargo-leptos is for SSR full-stack; trunk is correct for CSR/WASM SPA |
## Installation
### Frontend (Rust)
# Install Rust toolchain with WASM target
# Install trunk
# Cargo.toml dependencies
# [dependencies]
# leptos = { version = "0.8", features = ["csr"] }
# leptos_router = { version = "0.8", features = ["browser"] }
# leptos-use = "0.18"
# leptos-leaflet = "0.9"   # verify 0.8 compat first
# serde = { version = "1", features = ["derive"] }
# serde_json = "1"
# wasm-bindgen = "0.2"
# wasm-bindgen-futures = "0.4"
# web-sys = { version = "0.3", features = ["Window", "Storage"] }
# js-sys = "0.3"
### Build Step (Rust, for YAML → JSON conversion)
# Cargo.toml [build-dependencies] or separate binary crate
# serde = { version = "1", features = ["derive"] }
# serde-saphyr = "0.0.10"
# serde_json = "1"
### Scraping Pipeline (Python)
# Python 3.12+
## Open Questions / Flags for Implementation
## Sources
- [Leptos 0.8.17 on docs.rs](https://docs.rs/crate/leptos/latest)
- [Leptos Book: Deploying CSR Apps](https://book.leptos.dev/deployment/csr.html)
- [leptos-leaflet on crates.io](https://crates.io/crates/leptos-leaflet)
- [leptos-leaflet GitHub (headless-studio)](https://github.com/headless-studio/leptos-leaflet)
- [leptos-use changelog](https://leptos-use.rs/changelog.html)
- [Valhalla FOSSGIS server](https://valhalla.openstreetmap.de/)
- [Valhalla API docs](https://valhalla.github.io/valhalla/api/)
- [OSRM API usage policy](https://github.com/Project-OSRM/osrm-backend/wiki/Api-usage-policy)
- [Nominatim usage policy](https://operations.osmfoundation.org/policies/nominatim/)
- [serde-saphyr on crates.io](https://crates.io/crates/serde-saphyr)
- [Berlin Open Data WFS Schulen](https://daten.berlin.de/datensaetze/schulen-wfs-ebc64e18)
- [Berlin Eckdaten 2024/2025](https://daten.berlin.de/datensaetze/eckdaten-allgemeinbildende-schulen-2024-1506256)
- [Anthropic structured outputs docs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
