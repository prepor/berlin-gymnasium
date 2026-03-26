# Technology Stack

**Project:** Berlin Gymnasien Comparison Site
**Researched:** 2026-03-26

---

## Recommended Stack

### Frontend: Rust/Leptos SPA

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| leptos | 0.8.x (0.8.17 latest) | Reactive WASM SPA framework | Project constraint; fine-grained reactivity without virtual DOM diffing; CSR mode compiles to pure WASM SPA |
| leptos_router | bundled with 0.8.x | Client-side routing | Integrated with leptos, supports nested routes, query params for filter state |
| leptos-use | 0.18.x | Utility hooks (debounce, localStorage, URL state) | Equivalent of VueUse/react-use for leptos; use_debounce_fn for address input, use_local_storage for persisting filter state |
| trunk | latest stable | Build tool for CSR/WASM | Official recommended tool for leptos CSR; produces dist/ ready for GitHub Pages |
| wasm-bindgen | bundled via trunk | Rust-to-JS FFI | Required for calling browser APIs and JS libraries from Rust |

**Build mode:** CSR (Client-Side Rendering) only. Do not use cargo-leptos — that is for SSR/full-stack. Trunk is the correct tool for a pure WASM SPA.

**GitHub Pages deployment:** Run `trunk build --release --public-url /amman/` (or repo name), then copy `index.html` to `404.html` so deep-links work when users navigate directly to a route.

**Confidence:** HIGH — leptos 0.8.17 confirmed on docs.rs; trunk CSR deployment pattern confirmed in official leptos book.

---

### Map Library

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| leptos-leaflet | 0.9.x (latest; 0.7-branch for leptos 0.7, verify 0.8 compat) | Interactive school map | Idiomatic Leptos components wrapping Leaflet.js; provides MapContainer, Marker, TileLayer, Popup, Circle out of the box |
| Leaflet.js | 1.9.x (loaded via CDN or trunk asset) | Underlying map engine pulled by leptos-leaflet | Industry-standard raster tile map; ~40KB; works in all browsers; no WebGL requirement |
| OpenStreetMap tiles | — | Free map tiles | openstreetmap.org tile CDN; free; attribution required; adequate for ~90 markers |

**Why leptos-leaflet over MapLibre GL / kapta:**
- `leptos-leaflet` is the only production-tested Leptos-native map wrapper with active maintenance.
- MapLibre GL requires WebGL and is heavier; overkill for a 90-marker school locator.
- `kapta` is experimental/low-adoption.
- Leaflet raster tiles are sufficient for this use-case.

**Version caveat (MEDIUM confidence):** `leptos-leaflet` 0.9.1 targets leptos 0.6. A `leptos-0.7` branch exists on GitHub. Verify whether a leptos-0.8 compatible release exists before starting map work. If it lags, pin leptos to 0.7 or use raw `wasm-bindgen` to wrap Leaflet.js directly.

---

### Routing / Travel Time API

| Technology | Type | Purpose | Why |
|------------|------|---------|-----|
| Valhalla (valhalla1.openstreetmap.de) | Free public API | Travel time from user address to school (foot, transit, car) | Supports pedestrian, bicycle, auto, and multimodal transit costing; FOSSGIS-hosted public server; REST/JSON; OSM-based |
| Nominatim (nominatim.openstreetmap.org) | Free public API | Geocode user-entered address to lat/lng | CORS-enabled; no API key; 1 req/sec limit; sufficient for on-demand single geocodes |

**Why Valhalla over OSRM:**
- Valhalla supports foot, transit, and car natively in one API. OSRM only supports car/bicycle natively.
- OSRM public server has documented intermittent CORS issues. Valhalla's FOSSGIS server is more reliably CORS-enabled.
- Valhalla's `matrix` and `sources_to_targets` endpoints can compute travel times from one origin to multiple schools in a single call.

**Why not OpenRouteService:** OpenRouteService does not support CORS on its public endpoint, making direct browser calls impossible without a proxy.

**Usage pattern:**
1. User enters address → debounced Nominatim call → lat/lng
2. `POST https://valhalla1.openstreetmap.de/sources_to_targets` with user location as source and all 90 school coordinates as targets → returns a matrix of travel times in one request.
3. Results update school cards and map markers reactively.

**Rate limits:** 1 req/sec on Nominatim. Valhalla FOSSGIS: no hard documented limit but fair-use expected.

**Confidence:** MEDIUM — Valhalla FOSSGIS CORS status inferred from public demo app; not verified against response headers. Needs a quick test call before committing.

---

### Data Format: Rust YAML Parsing

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| serde | 1.x | Core serialization framework | Standard; used by all Rust data libraries |
| serde-saphyr | 0.0.10 | Deserialize YAML school files in build step | serde_yaml (dtolnay) is unmaintained. serde-saphyr is the fastest maintained replacement, passes full yaml-test-suite |

**Note:** YAML parsing happens at build/dev time (loading school data files), not in WASM. At runtime, ship the school data as a compiled-in `&str` constant or a JSON asset fetched at startup.

**Recommended pattern:**
- Build step: read `data/schools/*.yaml` → parse with serde-saphyr → serialize to a single `schools.json` → embed in WASM or place in `dist/` for fetch.
- WASM runtime: fetch `schools.json`, deserialize with `serde_json`.

**Confidence:** HIGH — serde_yaml deprecation confirmed on crates.io; serde-saphyr recommendation confirmed in Rust forum thread.

---

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

**Why not scrapy:** Scrapy is heavyweight for 90 schools. httpx + BeautifulSoup is simpler and sufficient.

**Pipeline architecture:**

```
Phase 1 → Seed (structured):
  Berlin Open Data WFS → GeoJSON → school list with coordinates, BSN numbers

Phase 2 → Enrich (scripted):
  schulverzeichnis.berlin.de → httpx + BS4 → admission criteria, languages, profile

Phase 3 → Enrich (AI agents):
  Claude + tool use → school websites → Ganztag, IB, open day dates, ratings
  Claude + structured output → extract JSON matching SchoolData pydantic model

Phase 4 → Validate + Write:
  Pydantic validation → write/update YAML files → generate diff changelog
```

**Claude Agent SDK vs raw anthropic SDK:**
Use the raw `anthropic` Python SDK with tool use, not the Claude Agent SDK. The Agent SDK is designed for interactive sessions. For a batch ETL pipeline, direct API calls with tool_use blocks give more control over parallelism, retry logic, and cost tracking.

**Confidence:** HIGH for Python + httpx + BeautifulSoup stack. MEDIUM for Claude structured outputs (quality needs validation). HIGH for ruamel.yaml round-trip.

---

### Berlin Open Data Source (Seed Data)

| Source | Format | What It Provides |
|--------|--------|-----------------|
| daten.berlin.de WFS (Schulen layer) | GeoJSON / WFS | School locations, coordinates, school type codes |
| daten.berlin.de "Eckdaten allgemeinbildende Schulen 2024/2025" | CSV/Excel | Student counts, teacher counts, school numbers (BSN) |
| JedeSchule.de | Web/data download | Structured school profiles, possibly filterable by Gymnasien |
| schulverzeichnis.berlin.de | HTML (scrape) | Admission criteria, languages, contact data |

The Berlin Open Data WFS is the canonical seed — it has GeoJSON export and filters by school type, making it straightforward to extract all Gymnasien with coordinates. This eliminates needing to geocode addresses for most schools.

**Confidence:** HIGH — daten.berlin.de WFS Schulen dataset confirmed as active with GeoJSON export capability.

---

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

---

## Installation

### Frontend (Rust)

```bash
# Install Rust toolchain with WASM target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk

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
```

### Build Step (Rust, for YAML → JSON conversion)

```bash
# Cargo.toml [build-dependencies] or separate binary crate
# serde = { version = "1", features = ["derive"] }
# serde-saphyr = "0.0.10"
# serde_json = "1"
```

### Scraping Pipeline (Python)

```bash
# Python 3.12+
pip install anthropic httpx beautifulsoup4 pydantic "ruamel.yaml" tenacity
```

---

## Open Questions / Flags for Implementation

1. **leptos-leaflet leptos 0.8 compatibility:** The latest published crate (0.9.1) targets leptos 0.6. Confirm whether a 0.8-compatible version exists. If not, either pin leptos to 0.7 or hand-write Leaflet.js wasm-bindgen bindings. **Verify before starting map phase.**

2. **Valhalla CORS confirmation:** Make a test `fetch()` from a browser to `https://valhalla1.openstreetmap.de/sources_to_targets` before committing to this routing approach. If blocked, fallback is OSRM for car-only routing.

3. **Google Maps rating scraping:** Google Maps ratings cannot be reliably scraped without violating ToS. Plan for Claude agent to extract visible rating text as a one-time data collection rather than ongoing scraping.

---

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
