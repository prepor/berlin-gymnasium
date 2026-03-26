# Project Research Summary

**Project:** Berlin Gymnasien Comparison Site (amman)
**Domain:** Static school comparison directory with data scraping pipeline
**Researched:** 2026-03-26
**Confidence:** MEDIUM

## Executive Summary

This project is a two-part system: a Python-based scraping pipeline that collects data on ~90 Berlin Gymnasien from multiple sources, and a Rust/Leptos WASM SPA deployed as a static site on GitHub Pages. The clean handoff point between them is a YAML data store — one file per school. Experts build this kind of information product by separating data collection (pipeline) from presentation (frontend) completely, with a well-typed schema contract between them. The pipeline enriches school records in passes: first from structured open data sources (Berlin Open Data WFS, schulverzeichnis.berlin.de), then from Claude-assisted agent research per school website.

The recommended approach centers on the `grundständig` flag (schools accepting students after 4th grade) as the defining differentiator — this is the primary reason a parent would choose this site over existing alternatives like gymnasium-berlin.net or the Tagesspiegel interactive map. The MVP must get school data, filtering, and an interactive map live first; travel time routing is the single highest-value differentiator and should come immediately after. Side-by-side comparison and enriched admission data are phase 3 targets.

The two most critical risks are: (1) the leptos-leaflet crate version compatibility with Leptos 0.8 must be verified before starting map work — if it is not compatible, the fallback is raw wasm-bindgen Leaflet bindings, which is significantly more effort; and (2) the routing API choice (Valhalla FOSSGIS vs. OSRM) requires a CORS test call from the browser before committing, because OSRM's public demo server prohibits production use and CORS support varies. Both must be validated in the first week of SPA development.

## Key Findings

### Recommended Stack

The frontend is a pure client-side Leptos 0.8 WASM SPA built with Trunk (not cargo-leptos, which is for SSR). School data is compiled into the binary at build time as embedded JSON — school YAML files are converted to a single `schools.json` in a pre-build step and embedded via `include_str!`. This avoids YAML parsing in WASM and runtime network fetches for the school corpus.

The scraping pipeline is Python 3.12+ using httpx for async HTTP, BeautifulSoup4 for HTML parsing, Pydantic v2 for schema validation, and the Anthropic SDK (raw tool use, not the Agent SDK) for per-school web research. ruamel.yaml handles round-trip YAML writes. The Berlin Open Data WFS is the canonical seed — it provides GeoJSON with coordinates and school type codes and is CC-BY 4.0 licensed.

**Core technologies:**
- Leptos 0.8 + Trunk (CSR): WASM SPA framework, GitHub Pages deployment
- leptos-leaflet + Leaflet.js: interactive school map (verify 0.8 compat before starting)
- Valhalla FOSSGIS API: travel time matrix for all routing modes (verify CORS before committing)
- Photon/komoot geocoder: user address lookup from browser (Nominatim is too restrictive for browser use)
- serde-saphyr: YAML parsing in Rust build step (serde_yaml is unmaintained)
- Python + httpx + BeautifulSoup4 + Anthropic SDK: scraping pipeline
- Pydantic v2 + ruamel.yaml: schema validation and round-trip YAML writes
- Berlin Open Data WFS: seed data (coordinates, school IDs, school type)

### Expected Features

**Must have (table stakes):**
- School listing with name, district, profile, grundständig flag — core directory
- Filter by district, profile, grundständig, Ganztag, and language — primary navigation
- School detail page with all fields (contact, ratings, languages, Ganztag, student-teacher ratio)
- Interactive map with clickable school pins
- Mobile-responsive layout (>60% of initial browsing is mobile)
- Fast initial load — WASM bundle size must be addressed from project start

**Should have (differentiators):**
- Travel time from user-entered address (walk + transit + car) — no existing Berlin school site offers this; highest-value feature
- Sort and filter by travel time
- Side-by-side comparison of 2–3 schools with URL encoding
- Open day (Tag der offenen Tür) dates with freshness timestamp — time-critical for Berlin's February Anmeldezeitraum
- Admission requirements per school (Notendurchschnitt threshold, oversubscription, Probeunterricht)
- Shareable permalink per school — parents share links in WhatsApp groups
- Data freshness indicator

**Defer (v2+):**
- Aggregated multi-source ratings — Google ratings thin and ToS-restricted; defer until permitted sources identified
- Changelog UI — pipeline generates it; surfacing in UI is polish
- IB / honors track flags — data availability uncertain
- Data freshness per-field — overhead without proven need

### Architecture Approach

The system has two wholly independent milestones with a YAML corpus as the handoff. The pipeline runs as a batch ETL job (GitHub Actions on a schedule), producing one YAML file per school. The SPA is a pure static CSR app that embeds the compiled school JSON at build time. The SPA has no server and no runtime API calls except for user-triggered travel time queries and address geocoding.

**Major components:**
1. Scraping pipeline (Python) — Seed Collector → Structured Scraper → Agent Researcher → Validator/Merger → Writer; asyncio with Semaphore(8) for concurrency
2. YAML data store (`data/schools/{school_id}.yaml`) — contract between pipeline and SPA; one Pydantic model defines the schema
3. Leptos WASM SPA — App shell + filter signals → School list view → Map view → Travel time widget → Comparison view; all school data embedded at build time
4. CI/CD (GitHub Actions) — runs pipeline → YAML → JSON → Trunk build → GitHub Pages deploy

### Critical Pitfalls

1. **Google Maps rating scraping violates ToS** — Do not scrape Google Maps. Use only explicitly permitted rating sources or omit ratings entirely for v1. Document what was excluded and why in the YAML schema.

2. **GitHub Pages 404 on non-root routes** — Commit to hash routing (`/#/schools/id`) in the first SPA scaffold commit. The 404.html redirect hack is fragile and breaks link previews. Retrofitting routing strategy after the SPA is built is painful.

3. **WASM binary bloat causes unacceptable load times** — Set `opt-level = "z"`, `lto = true`, `codegen-units = 1` in the release profile before adding any dependencies. Avoid the `regex` crate in WASM. Run `wasm-opt -Oz` on output. Measure with `twiggy` after each major dependency addition.

4. **OSRM public demo server prohibits production use** — Use the FOSSGIS-operated Valhalla server or OpenRouteService free tier. Never call `router.project-osrm.org` from production. Confirm CORS with a real browser test call before building the feature.

5. **Map DOM ownership conflict (Leaflet vs. Leptos)** — Isolate the map in a `NodeRef`-anchored div. Update markers imperatively via JS calls. Use CSS show/hide instead of conditional rendering for the map container. Initialize via `Effect::new` only after DOM mount.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Data Pipeline and Schema Foundation
**Rationale:** The YAML schema is the contract that all other work depends on. The pipeline must produce a complete, validated school corpus before the SPA can be built with real data. Getting structured data first (WFS + schulverzeichnis) provides 80% of the value with minimal cost before adding expensive Claude API calls.
**Delivers:** Complete, validated YAML corpus for all ~90 Berlin Gymnasien; reproducible pipeline with changelog; GitHub Actions scheduled run producing PRs
**Addresses:** School listing data, grundständig flag, district, profile, contact info, coordinates, Ganztag
**Avoids:** Google Maps ToS violation (Pitfall 1), data inconsistency across schools (Pitfall 5), GDPR exposure (Pitfall 10), stale data with no refresh mechanism (Pitfall 11)

### Phase 2: SPA Scaffold and Core Listing
**Rationale:** The SPA foundation — routing strategy, WASM size optimization, data loading — must be locked in before feature work begins. Retrofitting hash routing or Cargo.toml optimization profiles after the fact is expensive. This phase ships a working, filterable school directory.
**Delivers:** Deployed Leptos SPA on GitHub Pages with school listing, filters (district, profile, grundständig, Ganztag, language), school detail pages, mobile-responsive layout, and fast load time
**Uses:** Leptos 0.8 + Trunk, include_str! data embedding, leptos-use for debounce and localStorage
**Implements:** App shell, school list view, filter panel, detail view
**Avoids:** WASM binary bloat (Pitfall 4), GitHub Pages routing breakage (Pitfall 2), slow dev iteration (Pitfall 8)

### Phase 3: Interactive Map
**Rationale:** The map is a table stakes feature but has the most technical risk (leptos-leaflet version compatibility, DOM ownership conflict). It should be its own phase so the risk is isolated and the fix — if leptos-leaflet 0.8 compat is missing — does not block the listing view.
**Delivers:** OpenStreetMap map with clickable school pins; clicking a pin opens the school detail; map and list stay in sync with active filters
**Uses:** leptos-leaflet + Leaflet.js + OSM tile CDN
**Implements:** Map view component, NodeRef isolation pattern
**Avoids:** Map/Leptos DOM conflict (Pitfall 7); validate leptos-leaflet 0.8 compat before starting this phase

### Phase 4: Travel Time and Routing
**Rationale:** This is the single highest-value differentiator. It depends on school coordinates (Phase 1), a working SPA (Phase 2), and a verified routing API. CORS and rate-limit validation must happen at the start of this phase, not during.
**Delivers:** User address input → geocoding → travel time matrix for all schools (walk, transit, car) → sort and filter by travel time
**Uses:** Valhalla FOSSGIS sources_to_targets endpoint, Photon/komoot for address geocoding, leptos-use debounce
**Implements:** Travel time widget, address input, sort-by-travel-time signal
**Avoids:** OSRM demo server ToS violation (Pitfall 3), Nominatim browser restrictions (Pitfall 9)

### Phase 5: School Comparison and Data Depth
**Rationale:** Side-by-side comparison reuses the school card component from Phase 2 and the URL routing patterns already established. Open day dates and admission requirements are data-availability-constrained — the pipeline enrichment from Phase 1 determines what's possible here.
**Delivers:** Side-by-side comparison of 2–3 schools with shareable URL; open day calendar (with freshness date); admission requirements per school (Notendurchschnitt, oversubscription ratio, Probeunterricht flag); data completeness indicator per school
**Addresses:** Side-by-side comparison, open day dates, admission requirements, shareable permalinks
**Avoids:** Misleading rating aggregation (Pitfall 6) by showing only per-source attributed ratings

### Phase Ordering Rationale

- Phase 1 before Phase 2: The SPA can be scaffolded with mock data, but real data is needed to verify filter behavior, edge cases, and performance. Pipeline first reduces rework.
- Phase 2 before Phase 3: Hash routing and WASM size optimization are scaffold decisions. Map integration requires a stable app shell.
- Phase 3 before Phase 4: Travel time widget needs a map to display results on. Also: leptos-leaflet compat risk should be resolved before the routing API risk is introduced.
- Phase 4 before Phase 5: Comparison view benefits from having travel time already in the school data model. Users evaluating side-by-side always want to see commute times.
- Admission requirements depend on how complete the pipeline enrichment turned out in Phase 1 — Phase 5 can adjust scope based on actual data coverage.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1 (Pipeline):** Claude API tool use patterns for batch ETL, asyncio Semaphore tuning, actual field coverage achievable from schulverzeichnis.berlin.de
- **Phase 3 (Map):** leptos-leaflet 0.8 compatibility status — this is the top unresolved technical question; if incompatible, raw wasm-bindgen Leaflet bindings require significant research
- **Phase 4 (Travel time):** Valhalla FOSSGIS CORS verification, OpenRouteService free tier limits for transit routing, Photon geocoder accuracy for Berlin addresses

Phases with standard patterns (skip research-phase):
- **Phase 2 (SPA Scaffold):** Leptos CSR + Trunk deployment is well-documented in the official Leptos book; hash routing and Cargo.toml optimization are established patterns
- **Phase 5 (Comparison):** URL-encoded comparison state and side-by-side table layouts are standard SPA patterns

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH (with exceptions) | Leptos 0.8 + Trunk CSR deployment confirmed; serde-saphyr confirmed; leptos-leaflet 0.8 compat UNVERIFIED; Valhalla CORS UNVERIFIED |
| Features | MEDIUM | Core features verified against GreatSchools, Niche.com, gymnasium-berlin.net; Berlin-specific grundständig flow confirmed; ratings strategy needs permitted-sources audit |
| Architecture | HIGH | Two-system separation with YAML handoff is well-established; asyncio pipeline pattern is standard; data embedding strategy (include_str!) is confirmed Leptos CSR pattern |
| Pitfalls | HIGH | Google ToS, GitHub Pages routing, OSRM policy are documented facts; WASM size and map isolation are well-documented Leptos community issues |

**Overall confidence:** MEDIUM — the architecture and stack choices are solid, but two unverified technical dependencies (leptos-leaflet 0.8 compat, Valhalla CORS) must be confirmed before starting their respective phases.

### Gaps to Address

- **leptos-leaflet 0.8 compatibility:** Verify whether a Leptos 0.8 compatible release of leptos-leaflet exists on crates.io or the GitHub repo. If not, decide: pin Leptos to 0.7 or write raw wasm-bindgen Leaflet bindings. Resolve before Phase 3 planning.
- **Valhalla FOSSGIS CORS:** Make a real browser `fetch()` test to `https://valhalla1.openstreetmap.de/sources_to_targets` before finalizing Phase 4 architecture. Fallback is OpenRouteService free tier.
- **Permitted rating sources:** Audit whether any German school rating platforms (schulnoten.de, schulinfo.de, schulen.de) offer API access or permissive terms before designing the ratings display in Phase 2. If none are suitable, v1 ships without third-party ratings.
- **schulverzeichnis.berlin.de field coverage:** Run a sample scrape of 5–10 schools from schulverzeichnis.berlin.de during Phase 1 to confirm which enrichment fields are actually available before designing the full pipeline.

## Sources

### Primary (HIGH confidence)
- https://docs.rs/crate/leptos/latest — Leptos 0.8.17 confirmed
- https://book.leptos.dev/deployment/csr.html — Trunk CSR deployment pattern confirmed
- https://crates.io/crates/serde-saphyr — serde_yaml deprecation and serde-saphyr recommendation confirmed
- https://daten.berlin.de/datensaetze/schulen-wfs-ebc64e18 — Berlin WFS Schulen dataset confirmed as active, CC-BY 4.0
- https://cloud.google.com/maps-platform/terms — Google Maps ToS prohibition on scraping confirmed
- https://github.com/Project-OSRM/osrm-backend/wiki/Api-usage-policy — OSRM demo server policy confirmed
- https://operations.osmfoundation.org/policies/nominatim/ — Nominatim usage policy confirmed
- https://book.leptos.dev/deployment/binary_size.html — WASM binary size optimization patterns confirmed

### Secondary (MEDIUM confidence)
- https://crates.io/crates/leptos-leaflet — 0.9.1 targets leptos 0.6; 0.7 branch exists on GitHub; 0.8 compat unverified
- https://valhalla.openstreetmap.de/ — FOSSGIS Valhalla server exists; CORS support inferred from public demo app, not verified
- https://www.gymnasium-berlin.net/ — competitive feature landscape
- https://interaktiv.tagesspiegel.de/lab/alle-berliner-schulen-im-vergleich-2025-an-welchen-schulen-gibt-es-die-besten-abiturnoten-interaktive-karte/ — Tagesspiegel interactive map as competitive reference

### Tertiary (LOW confidence)
- schulnoten.de, schulinfo.de — existence known; ToS and API availability not verified; needs audit before ratings feature design

---
*Research completed: 2026-03-26*
*Ready for roadmap: yes*
