# Domain Pitfalls

**Domain:** Berlin Gymnasien comparison website (data scraping pipeline + Rust/Leptos WASM SPA)
**Researched:** 2026-03-26

---

## Critical Pitfalls

Mistakes that cause rewrites, legal exposure, or fundamental feature failure.

---

### Pitfall 1: Scraping Google Maps Ratings Violates Google's Terms of Service

**What goes wrong:** The project wants to aggregate ratings from Google Maps. Google's Terms of Service explicitly prohibit automated extraction of Google Maps Content, including business ratings and reviews. Scraping is detected and blocked; IPs get banned; in commercial contexts it exposes to legal enforcement.

**Why it happens:** The rating data looks like public information, so developers assume it is freely scrapable.

**Consequences:** Google blocks the scraper after a short time. Automated scraping of Google Maps is technically a contractual violation (not criminal), but Google enforces aggressively through technical means. More importantly, the scraped data may not be redistributed in a static site — the Places API ToS explicitly prohibits displaying Google review content outside Google's own service.

**Prevention:**
- Do not scrape Google Maps ratings at all. Use only sources that explicitly permit aggregation: school comparison sites, or aggregators that provide an API.
- Investigate whether any German school rating platforms (e.g., schulnoten.de, schulinfo.de) provide structured data or permissive terms.
- Fall back to displaying "no third-party rating" and showing only official/self-reported data for schools that have no permitted rating source.
- Document the data-sourcing decisions in the YAML schema so the absence of a rating field is explicit.

**Detection:** The scraper returns correct results the first few runs, then begins returning empty or challenge pages.

**Phase:** Data pipeline (Phase 1 / data collection). Must be resolved before building the aggregation logic.

---

### Pitfall 2: GitHub Pages Breaks All Non-Root Routes (SPA 404)

**What goes wrong:** GitHub Pages serves static files only. When a user navigates directly to `/schools/sch-name` or refreshes, GitHub Pages returns a real HTTP 404 because no file exists at that path. The Leptos SPA never boots.

**Why it happens:** Client-side routing works by intercepting navigation inside the running app. On direct access GitHub Pages bypasses the app entirely.

**Consequences:** Every shareable link and every browser refresh on a non-root URL is broken. This is especially bad for the comparison mode (users want to share `/compare?a=x&b=y`).

**Prevention:** Two approaches, each with tradeoffs:
1. **Hash routing** — use `/#/schools/name` style routes. Simple, always works. Downside: ugly URLs, conflicts with anchor hash semantics, bad for link previews.
2. **404.html redirect hack** — copy `index.html` to `404.html`, or use the standard `spa-github-pages` trick where `404.html` encodes the path into a query string and redirects. This preserves clean URLs but pages still technically return 404 initially, which breaks social link previews and can show browser warnings.

**Recommendation:** Use hash routing. This is a school comparison tool, not a marketing site — SEO and link previews for deep routes are not critical. Hash routing is zero-configuration and completely reliable on GitHub Pages. Decide this in Phase 1 (project scaffold) and commit; retrofitting routing strategy after the SPA is built is painful.

**Detection:** Works locally (dev server handles any route), breaks after first deploy to GitHub Pages.

**Phase:** SPA scaffold (Phase 2). Must be baked into the initial Leptos router setup.

---

### Pitfall 3: OSRM Public Demo Server Is Not Suitable for Production Use

**What goes wrong:** The project needs client-side travel time estimation. The obvious approach is to call the OSRM public demo server (`router.project-osrm.org`) from the browser. This works during development but breaks at scale or gets the app blocked.

**Why it happens:** The OSRM demo server's usage policy explicitly limits use to 1 request per second, forbids integration into production applications, and can block IPs without notice. Additionally CORS support is inconsistent.

**Consequences:** Travel time feature stops working for any real user load. The app silently fails or shows errors. There is no SLA or uptime guarantee on the demo server.

**Prevention:**
- Use `routing.openstreetmap.de` (FOSSGIS-operated OSRM instance) which explicitly supports public applications with proper attribution, still with 1 req/sec limit.
- Plan from the start to rate-limit calls: only compute travel time on explicit user action ("Calculate travel time"), never auto-compute for all 90 schools simultaneously.
- Alternatively, consider OSRM self-hosting on a cheap VPS, or use the Valhalla routing engine which has a public API from Stadia Maps / OpenRouteService with higher quotas and documented CORS support.
- OpenRouteService (`openrouteservice.org`) offers a free tier with 2,000 requests/day and proper CORS headers — better suited for a static site.

**Detection:** Works from `localhost` or with a browser extension disabling CORS checks; fails in production. Rate limit hit during peak usage.

**Phase:** SPA travel-time feature (Phase 3 or later). Architecture decision must happen before implementing the feature.

---

### Pitfall 4: WASM Bundle Is Too Large for Acceptable Initial Load

**What goes wrong:** An unoptimized Leptos WASM binary for a non-trivial app with serde, routing, map integration, and 90+ school objects embedded can easily reach 3–5 MB uncompressed. On mobile networks this causes a blank screen for 5–10 seconds before anything appears.

**Why it happens:** Several compounding factors: debug build artifacts in release, serde's code generation is large, regex crate adds ~500KB alone, missing Cargo.toml size-optimization profile.

**Consequences:** Poor first-impression performance. Since GitHub Pages does not support Brotli compression automatically (only gzip via CDN workarounds), the compressed size may still be 800KB–1.5MB.

**Prevention:**
- Add a release profile in `Cargo.toml` with `opt-level = "z"`, `lto = true`, `codegen-units = 1` from the project start.
- Avoid the `regex` crate entirely in WASM code; use browser's built-in regex via `web_sys` if needed.
- Use `miniserde` or `serde-lite` for WASM resource serialization instead of full `serde`.
- Run `wasm-opt -Oz` (via `wasm-pack` or `cargo-leptos`) on the output binary.
- Measure binary size with `twiggy` before and after adding each major dependency.
- Embed school data as a static JSON blob compiled into the binary at build time (`include_str!`) rather than fetching 90 separate YAML files at runtime.

**Detection:** Binary size reported by `cargo leptos build --release` exceeds 2MB uncompressed.

**Phase:** SPA scaffold (Phase 2). The optimization profile must be set up before adding features; retrofitting is harder.

---

## Moderate Pitfalls

---

### Pitfall 5: German School Website Data Is Structurally Inconsistent

**What goes wrong:** Each of the ~90 Gymnasien has an independently managed website. Formats vary wildly: some use PDF for everything, some have accessible HTML tables, some have not updated their site since 2015. Data points that should be scraped (teacher count, admission criteria, Notendurchschnitt) may not be published at all or may be buried in unstructured prose.

**Why it happens:** Berlin school websites are run by individual schools with no central CMS mandate. The official `schulverzeichnis.berlin.de` provides basic directory data (address, contact, type), but not the richer profile data needed for comparison.

**Consequences:** The scraping pipeline produces inconsistent coverage: some schools have 20 data fields, others have 3. The comparison feature becomes misleading when users compare a fully-populated school against one with blanks.

**Prevention:**
- Prioritize the Berlin Open Data WFS/WMS school dataset (available at `daten.berlin.de`) for baseline data — it is structured and CC-BY 4.0 licensed.
- For the richer fields, design the YAML schema with explicit `null` vs. `unknown` vs. a value distinction. Never leave a field ambiguous.
- In the SPA, surface data completeness: show a "data completeness" indicator per school so users know when a comparison is incomplete.
- Run the pipeline in two passes: (1) structured sources first, (2) school website scraping to fill gaps. This prevents the pipeline from treating a scraping failure as a data gap when the data simply doesn't exist.

**Detection:** More than 30% of schools have fewer than 50% of fields populated after pipeline run.

**Phase:** Data pipeline design (Phase 1).

---

### Pitfall 6: Rating Aggregation Across Different Scales Produces Misleading Scores

**What goes wrong:** If ratings from different sources (e.g., a 5-star scale, a 10-point scale, a thumbs-up/down site) are naively averaged or displayed side-by-side, users will misinterpret the data. A 4.0/5 and 4.0/10 look identical but mean very different things.

**Why it happens:** Developers treat rating as a generic number without encoding its scale and source provenance.

**Consequences:** Users make school decisions based on incorrectly interpreted ratings. Trust in the product erodes.

**Prevention:**
- In the YAML schema, store each rating with its source name, scale (min/max), raw value, and date scraped. Never store a pre-normalized number.
- Display ratings source-by-source with explicit scale labels, not an aggregated single score.
- If computing an aggregate, normalize to a 0–100 scale and label it as "normalized score" with a tooltip explaining the methodology.
- Flag sources with very few reviews (< 5 ratings) as statistically unreliable.

**Detection:** YAML files contain bare numeric rating fields without source attribution metadata.

**Phase:** YAML schema design (Phase 1) and SPA display (Phase 2–3).

---

### Pitfall 7: JavaScript/WASM Interop for Maps Is Fragile

**What goes wrong:** Leaflet and MapLibre are JavaScript libraries. Integrating them into a Leptos WASM app requires calling JS from Rust via `wasm-bindgen` and `web_sys`. The map library manipulates the DOM directly; Leptos also controls the DOM. This creates state conflicts: Leptos re-renders may destroy the map container or orphan event listeners.

**Why it happens:** Both frameworks assume they are the single source of truth for DOM state in their region.

**Consequences:** Map flickers or disappears on reactive state changes. Marker clicks stop firing. The map container gets unmounted and remounted unexpectedly.

**Prevention:**
- Use `leaflet-rs` (wasm-bindgen wrapper for Leaflet) rather than raw JS interop — it provides typed bindings and handles some lifecycle issues.
- Isolate the map into a non-reactive container: render the map inside a `NodeRef`-anchored div and update it imperatively via JS calls rather than letting Leptos re-render it.
- Use Leptos's `Effect::new` to initialize the map only after the DOM node is mounted, not in the component body.
- Never let Leptos reactive signals directly control the map container's presence in the DOM — use CSS show/hide instead of conditional rendering for the map div.

**Detection:** Map disappears when applying a filter; school pins stop responding to clicks after navigating away and back.

**Phase:** Map feature implementation (Phase 2–3). Plan the isolation strategy before writing the first map component.

---

### Pitfall 8: Rust/WASM Development Iteration Is Slow Without Proper Setup

**What goes wrong:** Cold Rust compilation for a WASM target takes 2–5 minutes on a typical laptop. Without a proper incremental build setup, each change requires a full rebuild, killing developer productivity.

**Why it happens:** Rust compiles the entire dependency tree from scratch without caching. WASM target is a separate compilation unit from native Rust.

**Consequences:** The SPA takes disproportionately long to build. Developers avoid making small improvements or tests because each iteration is costly.

**Prevention:**
- Use `cargo-leptos watch` mode from day one — it handles incremental WASM recompilation and hot-reload.
- Set up `sccache` for build artifact caching, especially if running in CI.
- Keep the dependency footprint minimal — each new crate adds to compile time. Audit dependencies before adding.
- Split the school data out of the binary if it's large — changes to data should not trigger WASM recompilation.

**Detection:** Build times exceed 3 minutes for an incremental change to a single component file.

**Phase:** Project scaffold (Phase 2). Build tooling must be configured before heavy feature work begins.

---

## Minor Pitfalls

---

### Pitfall 9: Nominatim Geocoding Cannot Be Called Directly from the Browser for Address Lookup

**What goes wrong:** The travel time feature requires geocoding a user-entered address to get coordinates before calling the routing API. Nominatim (OSM's geocoder) prohibits autocomplete-style queries from client-side code and enforces a 1 req/sec limit with a valid User-Agent header — which browsers do not send by default.

**Prevention:**
- Pre-geocode all school addresses during the scraping pipeline and store coordinates in YAML — avoid runtime school geocoding entirely.
- For user address input, use a debounced single lookup (not autocomplete) through `nominatim.openstreetmap.org` with proper Referer attribution, or use the Photon geocoder (`photon.komoot.io`) which is more permissive for web use.
- Alternatively, Komoot's Photon geocoder (`https://photon.komoot.io/api/`) is open, CORS-enabled, and does not require registration.

**Detection:** Address lookup works locally but fails in production due to missing User-Agent or rate limit.

**Phase:** Travel time feature (Phase 3+).

---

### Pitfall 10: DSGVO/GDPR Does Not Block Scraping of Institutional Public Data

**What goes wrong:** Developers either over-restrict themselves (avoid all scraping for fear of GDPR) or under-restrict (assume all public data is freely usable). Both are wrong for this project.

**Why it happens:** GDPR applies to personal data of natural persons, not to institutional/organizational data. School names, addresses, phone numbers, and institutional email addresses published on official school websites are not personal data of natural persons — they are institutional data.

**Consequences:** Over-restriction: project never starts. Under-restriction: scraping teacher names, individual email addresses, or student information creates genuine liability.

**Prevention:**
- Scrape only institutional data: school name, address, institutional phone, institutional email, school type, offered programs. This is clearly not personal data under GDPR Art. 4.
- Do not scrape: individual teacher names, personal email addresses, or any data about students.
- For the Berlin Open Data portal datasets, the data is CC-BY 4.0 licensed — explicitly permitted and citable.
- Add a `data_sources` section to each YAML file documenting the origin of each field for auditability.

**Detection:** YAML files contain fields that could identify natural persons (e.g., `principal_name`).

**Phase:** Data pipeline design (Phase 1). Schema design is the enforcement point.

---

### Pitfall 11: School Data Goes Stale Immediately After Scraping

**What goes wrong:** Berlin Gymnasien change admission criteria, open day dates, and contact info regularly. A static site with data scraped once will show outdated information. The project requires a reproducible pipeline with changelog, but if the changelog is never reviewed, staleness goes unnoticed.

**Prevention:**
- Design the YAML schema to include a `last_scraped` timestamp per field or per school.
- The pipeline changelog should diff field-level changes, not just file-level. A changed phone number should be visible as `phone: "030-old" → "030-new"` in the changelog.
- Automate the pipeline run via GitHub Actions on a schedule (e.g., monthly), with the changelog output as a PR for human review before merging to `main`.
- The SPA should display a "data last updated: [date]" notice so users understand freshness.

**Detection:** Committed YAML files have `last_scraped` dates older than 3 months with no changelog entry explaining why.

**Phase:** Data pipeline design (Phase 1) and CI setup.

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|----------------|------------|
| YAML schema design | Missing provenance fields (source, scale, date) for ratings | Treat ratings as objects `{source, raw, scale, scraped_at}` from day one |
| Data collection: Google ratings | ToS violation, IP ban | Use only permitted sources; document what was excluded and why |
| SPA router setup | GitHub Pages 404 on direct routes | Commit to hash routing in the first scaffold commit |
| Cargo.toml and build setup | WASM binary bloat from serde, regex | Set size-optimization release profile before adding dependencies |
| Map component | DOM ownership conflict between Leptos and Leaflet | Isolate map in a NodeRef div, update imperatively |
| Travel time feature | OSRM demo server rate limits; CORS failures | Use OpenRouteService free tier or FOSSGIS server with 1 req/sec limit |
| User address geocoding | Nominatim browser restrictions | Use Photon/komoot for user-side geocoding |
| Pipeline reproducibility | Changelog granularity too coarse to be useful | Diff at field level, not file level |
| Data freshness | No mechanism to re-run and review | GitHub Actions scheduled run → PR with changelog |

---

## Sources

- [Leptos: Optimizing WASM Binary Size](https://book.leptos.dev/deployment/binary_size.html)
- [Leptos: Integrating with JavaScript (wasm-bindgen, web_sys)](https://book.leptos.dev/web_sys.html)
- [GitHub Community: GitHub Pages does not support routing for SPAs](https://github.com/orgs/community/discussions/64096)
- [rafgraph/spa-github-pages: SPA GitHub Pages hack](https://github.com/rafgraph/spa-github-pages)
- [OSRM API Usage Policy](https://github.com/Project-OSRM/osrm-backend/wiki/Api-usage-policy)
- [Nominatim Usage Policy](https://operations.osmfoundation.org/policies/nominatim/)
- [Google Maps Terms of Service](https://cloud.google.com/maps-platform/terms)
- [Berlin Open Data: Schulen WFS dataset](https://daten.berlin.de/datensaetze/schulen-wfs-ebc64e18)
- [leaflet-rs: wasm-bindgen wrapper for Leaflet.js](https://github.com/slowtec/leaflet-rs)
