# Architecture Patterns

**Domain:** Data scraping pipeline + static WASM SPA (Berlin Gymnasien comparison)
**Researched:** 2026-03-26

---

## System Overview

Two wholly independent systems with a clean handoff point — a YAML data store:

```
[M1: Scraping Pipeline]          [Handoff]          [M2: WASM SPA]

  Sources                        data/              Browser
  ───────                        schools/           ───────
  Berlin Open Data WFS ───┐      ├── 01k01.yaml     index.html
  schulverzeichnis.de     ├──► Merge & Validate ──►  app.wasm
  School websites         │      ├── 01k02.yaml     schools.json (embedded)
  Google Maps ratings     │      ├── ...
  jedeschule API ─────────┘
```

The pipeline never touches the SPA. The SPA never touches the pipeline. The YAML directory is the contract between them.

---

## Milestone 1: Scraping Pipeline

### Component Boundaries

| Component | Responsibility | Inputs | Outputs |
|-----------|---------------|--------|---------|
| **Orchestrator** (`pipeline/run.py`) | Spawn and coordinate agents, manage concurrency limits | Config, school list seed | Exit code, run log |
| **Seed Collector** | Build canonical list of all Berlin Gymnasien with IDs | Berlin WFS endpoint, jedeschule API | `data/schools_index.yaml` |
| **Structured Scraper** | Pull machine-readable fields (address, coords, student count, phone) | WFS GeoJSON, schulverzeichnis CSV | Raw field dicts per school |
| **Agent Researcher** | Web research per school — ratings, open day, 4th-grade acceptance, profile details | School name + website URL | Unvalidated field dicts per school |
| **Validator / Merger** | Merge structured + agent data, validate schema, flag conflicts | Raw dicts from both scrapers | Validated `SchoolRecord` objects |
| **Writer** | Serialize to YAML, compute diff vs prior run, write changelog | `SchoolRecord` objects, prior YAML files | `data/schools/*.yaml`, `data/CHANGELOG.md` |

### Data Flow

```
Step 1 — Seed (sequential, one-time):
  WFS endpoint + jedeschule API → Seed Collector → schools_index.yaml
  (canonical list: school_id, name, district, website)

Step 2 — Parallel scrape (concurrent, bounded):
  For each school_id in schools_index.yaml:
    ├── Structured Scraper (httpx + BeautifulSoup) ──────────────┐
    └── Agent Researcher (Claude API, async)                      ├──► raw_data[school_id]
                                                                   │
  asyncio.gather with semaphore (max ~10 concurrent API calls)    │
  Structured scraper and agent run concurrently per school        │

Step 3 — Merge + Validate (per-school, can be parallel):
  raw_data[school_id] → Validator → SchoolRecord (pydantic model)
  Conflicts flagged as warnings; structured source wins over agent

Step 4 — Write (sequential):
  For each SchoolRecord:
    ├── Serialize to YAML (ruamel.yaml, preserves comments)
    ├── Diff against existing file (yaml-diff / unified diff)
    └── Append changes to CHANGELOG.md with timestamp + field name
```

### Concurrency Model

Use Python `asyncio` with a `Semaphore` to cap concurrent Claude API calls (recommended: 5–10). Structured HTTP scraping uses `httpx.AsyncClient` sharing the same event loop. Do not use threading — async is sufficient for I/O-bound work and avoids GIL complications with the Claude SDK.

```python
# Sketch — not final code
sem = asyncio.Semaphore(8)

async def scrape_school(school_id, meta):
    async with sem:
        structured = await scrape_structured(meta)
        agent_data = await run_agent(meta)
        return merge_and_validate(structured, agent_data)

results = await asyncio.gather(*[scrape_school(id, m) for id, m in index.items()])
```

### YAML Schema Contract

One file per school: `data/schools/{school_id}.yaml`

The Pydantic model defines the canonical schema. All fields optional except `school_id`, `name`, `district`. Missing fields left absent (not `null`) so diffs are clean.

```yaml
# data/schools/01k01.yaml
school_id: "01k01"
name: "Gymnasium am Beispiel"
district: "Mitte"
address: "Musterstraße 1, 10115 Berlin"
coords:
  lat: 52.5200
  lng: 13.4050
website: "https://www.gym-beispiel.de"
phone: "+49 30 12345"
accepts_after_4th_grade: true
profile:
  - bilingual_english
  - MINT
ganztag: true
student_count: 820
teacher_count: 64
languages:
  - {name: "Englisch", from_grade: 5}
  - {name: "Französisch", from_grade: 7}
ratings:
  google_maps:
    score: 4.2
    count: 87
    retrieved: "2026-03-26"
open_day: "2026-04-12"
notes: "Probeunterricht im März"
data_sources:
  - wfs_berlin
  - agent_research
last_updated: "2026-03-26"
```

Key schema decisions:
- `accepts_after_4th_grade: bool` is a first-class field — most important filter for parents
- `profile` is a list of enum strings, not free text — enables filtering
- `ratings` is a nested object keyed by source so multiple rating sources coexist cleanly
- `data_sources` records provenance per record for auditability
- `last_updated` enables incremental re-runs (skip schools not changed)

### Changelog Pattern

After each pipeline run, `data/CHANGELOG.md` is appended (not replaced):

```markdown
## 2026-03-26T14:32:00Z

### 01k01 — Gymnasium am Beispiel
- `accepts_after_4th_grade`: null → true
- `ratings.google_maps.score`: 4.1 → 4.2

### 01k02 — Gymnasium Neukölln
- NEW RECORD
```

Implementation: load prior YAML, load new YAML, run field-level diff, write human-readable summary. Use `deepdiff` Python library for nested dict diff.

---

## Milestone 2: Rust/Leptos WASM SPA

### Component Boundaries

| Component | Responsibility | Technology |
|-----------|---------------|------------|
| **Build preprocessor** (`build.rs` or CI step) | Convert `data/schools/*.yaml` → single `schools.json`, embed into WASM binary | Rust `build.rs` + `serde_yaml` + `include_str!` |
| **App shell** | Route between views, global state (filters, selected schools, user address) | Leptos CSR, `leptos_router` |
| **School list view** | Filterable, sortable table/card grid of all schools | Leptos reactive signals |
| **Filter panel** | District, profile, 4th-grade flag, Ganztag, language filters | Leptos signals → derived signal |
| **Map view** | Pin all schools on OSM map, click to detail | `leptos_leaflet` crate + Leaflet.js |
| **Travel time widget** | Accept address input, call OSRM API, show results per school | `wasm-bindgen` fetch → OSRM route matrix |
| **Detail / comparison view** | Side-by-side 2–3 schools, all fields | Leptos reactive component |
| **Deployment** | Static hosting, no server | GitHub Pages via GitHub Actions + Trunk |

### Data Embedding Strategy

All school data is embedded at build time — no runtime API calls for school data.

**Pattern:** Pre-build step converts YAML → JSON, then Rust `include_str!` embeds JSON into the WASM binary.

```
data/schools/*.yaml
        │
        ▼ (CI step: python scripts/build_data.py)
src/generated/schools.json    (~90 schools × ~50 fields → ~200KB uncompressed)
        │
        ▼ (build.rs or inline)
const SCHOOLS_JSON: &str = include_str!("../src/generated/schools.json");
        │
        ▼ (app startup, once)
let schools: Vec<School> = serde_json::from_str(SCHOOLS_JSON).unwrap();
```

Why JSON not YAML in the binary: `serde_json` is smaller and faster to parse in WASM than YAML parsers. Convert once at build time.

Size concern: 90 schools × ~200 bytes of JSON each → ~18KB → trivially small. Even at 2KB per school (all fields) = 180KB, acceptable for a static asset that is cached after first load.

### Travel Time: Client-Side via OSRM Public API

```
User enters address
        │
        ▼ (geocode via Nominatim/OSM free API)
{lat, lng} of user address
        │
        ▼ (OSRM route matrix endpoint)
GET https://router.project-osrm.org/table/v1/driving/{user_lng},{user_lat};{school1_lng},{school1_lat};...
        │
        ▼ (parse durations[][] from JSON response)
Display travel times per school (walking, driving, transit*)
```

OSRM public endpoint is free with fair-use limits. For 90 schools per user query, batch all coordinates into a single matrix call — OSRM table endpoint handles this efficiently.

Transit (public transport) requires a different API — OSRM does not support transit routing. Use OpenRouteService free tier for transit: it supports `public-transport` profile with a free API key (2000 requests/day).

Recommended: offer foot + driving via OSRM (no auth required), transit via OpenRouteService (user may hit rate limit for heavy use — acceptable trade-off for a static site).

### Map: leptos-leaflet

The `leptos_leaflet` crate provides native Leptos components wrapping Leaflet.js. This is the highest-confidence path for OSM maps in a Leptos app.

```rust
<MapContainer center=Position::new(52.52, 13.40) zoom=11.0>
    <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"/>
    {schools.iter().map(|s| view! {
        <Marker position=Position::new(s.lat, s.lng)>
            <Popup>{s.name.clone()}</Popup>
        </Marker>
    }).collect_view()}
</MapContainer>
```

### SPA Routing

Use `leptos_router` for client-side routing. GitHub Pages requires a 404.html redirect trick for deep links. Routes:

```
/                   → School list (default filters)
/school/:id         → School detail
/compare            → Comparison view (up to 3 schools, stored in URL query params)
/map                → Map view
```

State shared across routes via Leptos context (not URL) for filter state and user address.

### Build and Deployment

```
Trunk build --release
        │
        ▼
dist/
  ├── index.html
  ├── app-[hash].wasm
  ├── app-[hash].js
  └── app-[hash].css

GitHub Actions:
  1. run python scripts/build_data.py  (YAML → JSON)
  2. trunk build --release
  3. deploy dist/ to gh-pages branch
```

---

## Suggested Build Order (Dependency Chain)

```
1. YAML Schema definition (Pydantic model)
   ──► All other components depend on this contract

2. Seed Collector
   ──► Provides school_id list for all subsequent work

3. Structured Scraper (WFS + schulverzeichnis)
   ──► Baseline data; can proceed without agent data

4. Agent Researcher
   ──► Enrichment layer; depends on school list from step 2

5. Validator / Merger + Writer
   ──► Depends on 3 + 4; produces YAML output

6. Build preprocessor (YAML → JSON)
   ──► Depends on complete YAML corpus from step 5

7. Leptos SPA — data layer (load + deserialize schools.json)
   ──► Depends on step 6 producing valid JSON

8. Leptos SPA — list + filter view
   ──► Depends on step 7

9. Leptos SPA — map view
   ──► Depends on step 7; parallel with step 8

10. Leptos SPA — travel time widget
    ──► Depends on school coords from step 7; can be added after basic list works

11. Leptos SPA — comparison view
    ──► Depends on step 8 (reuses school cards)

12. GitHub Actions CI/CD pipeline
    ──► Final integration: runs steps 1–11 end-to-end
```

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Embedding YAML Directly in WASM
**What:** Using `include_str!` on raw YAML files and parsing with `serde_yaml` in-browser.
**Why bad:** `serde_yaml` adds significant binary size; YAML parsing in WASM is slower than JSON; the YAML schema is meant for human editing, not runtime parsing.
**Instead:** Convert YAML → JSON as a pre-build step. Embed JSON. Parse with `serde_json`.

### Anti-Pattern 2: One Giant Monolithic Scraper
**What:** Single script that scrapes, validates, and writes sequentially.
**Why bad:** 90 schools × multiple sources × sequential = hours of wall time; any failure restarts everything.
**Instead:** Parallel asyncio with per-school idempotency (skip if `last_updated` is today and source hasn't changed).

### Anti-Pattern 3: Mutable Agent State (Race Conditions)
**What:** Multiple agents writing directly to shared `data/schools/` during scraping.
**Why bad:** Concurrent file writes corrupt YAML; partial writes make diff/changelog unreliable.
**Instead:** Agents return data in-memory; single Writer component serializes all YAML at the end.

### Anti-Pattern 4: Server-Side Rendering for This Use Case
**What:** Using SSR mode of Leptos (cargo-leptos + Axum) to render school pages.
**Why bad:** Requires a server, eliminates GitHub Pages deployment, adds operational complexity.
**Instead:** CSR with Trunk. All data is static, CSR + `include_str!` is the correct fit.

### Anti-Pattern 5: Geocoding at Runtime in the SPA
**What:** Storing only addresses in YAML, resolving to lat/lng in the browser.
**Why bad:** Requires runtime API call before map renders; adds latency and API dependency.
**Instead:** Geocode during the scraping pipeline (use Nominatim). Store `coords.lat`/`coords.lng` in YAML.

---

## Scalability Considerations

| Concern | For 90 schools (current) | If scaled to 500+ schools |
|---------|--------------------------|---------------------------|
| YAML corpus size | ~100–200KB total, trivial | ~1MB+ — still fine for static deploy |
| Scraping wall time | ~10–20 min with parallelism | Linear — just add capacity |
| WASM binary size | schools.json adds ~50–200KB | Still well under 2MB budget |
| OSRM matrix call | 90 coords in one batch call | 500 coords may exceed URL length — split into chunks |
| Filter performance | In-memory JS/WASM, instant | In-memory up to ~5K records without virtualization |
| Map marker count | 90 markers — no issue | 500+ may need clustering (Leaflet.markercluster) |

---

## Sources

- Leptos CSR deployment guide: https://book.leptos.dev/deployment/csr.html
- leptos-leaflet crate: https://docs.rs/leptos-leaflet/latest/leptos_leaflet/
- OSRM public API: https://project-osrm.org/docs/v5.24.0/api/
- OpenRouteService JS client: https://github.com/GIScience/openrouteservice-js
- Berlin Open Data WFS school layer: https://daten.berlin.de/datensaetze/schulen-wfs-ebc64e18
- jedeschule scraper (existing prior art): https://github.com/Datenschule/jedeschule-scraper
- Pydantic YAML validation: https://pypi.org/project/pydantic-yaml/
- leptos-rs/start-trunk (CSR starter): https://github.com/leptos-rs/start-trunk
