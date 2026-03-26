# Phase 2: SPA Foundation and School Directory - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Leptos WASM SPA deployed to GitHub Pages with a filterable school listing and detail pages. Parents can browse all 106 Berlin Gymnasien, filter by district/profile/grundständig/language/Ganztag, and click into a detail page showing all available data fields. Mobile-responsive.

Requirements: LIST-01 through LIST-07, DETL-01 through DETL-06, DEPL-01 through DEPL-03.

</domain>

<decisions>
## Implementation Decisions

### Visual Design
- **D-01:** Clean card grid layout — each school is a card showing key info at a glance
- **D-02:** Mobile-first responsive design — cards stack vertically on phone, 2-3 columns on desktop
- **D-03:** Utility CSS approach (inline styles or minimal CSS) — no heavy framework, keep WASM bundle small
- **D-04:** German-language UI — target audience is Berlin parents; all labels, filters, and content in German

### Filter UI
- **D-05:** Sidebar filters on desktop, collapsible top panel on mobile
- **D-06:** Filters: District (multi-select), Profile (multi-select), Grundständig (yes/no/all), Language (multi-select), Ganztag (yes/no/all)
- **D-07:** Filters combine with AND logic (narrow results)
- **D-08:** Active filter count shown as badge; clear-all button
- **D-09:** Filter state persisted in URL hash params so filtered views are shareable

### School Card Content
- **D-10:** Card shows: school name, district, profile badges (colored chips), grundständig flag (prominent if yes), student count, completeness score indicator
- **D-11:** Cards are clickable — navigate to detail page
- **D-12:** Sort options: alphabetical, by district, by student count (default: alphabetical)

### Detail Page
- **D-13:** Detail page sections in order: Hero (name, district, address, website link) → Profile & Languages → Admission Requirements → Ratings → Open Day → Contact → Data Provenance (sources, last updated, confidence)
- **D-14:** Missing data shown as "Keine Angabe" (no information) rather than hiding the section — transparency over aesthetics
- **D-15:** Back button returns to listing with filters preserved
- **D-16:** External links (website, email, phone) open in new tab / native handler

### Routing & Deployment
- **D-17:** Hash routing (`/#/`, `/#/school/{id}`) — simplest for GitHub Pages, no 404.html hack needed
- **D-18:** Routes: `/#/` (listing with filters), `/#/school/{school_id}` (detail page)
- **D-19:** Data embedded at build time: build.rs reads `data/schools/*.yaml` → serializes to single JSON string → `include_str!` in WASM binary
- **D-20:** WASM optimization profile: `opt-level = "z"`, `lto = true`, `codegen-units = 1` in Cargo.toml release profile
- **D-21:** GitHub Actions workflow: trunk build → deploy to gh-pages branch
- **D-22:** Trunk config: `trunk build --release --public-url /amman/` (repo name)

### Claude's Discretion
- Exact CSS styling and color palette — pick something clean and professional
- Component decomposition — how to split Leptos components
- Whether to use leptos-use for URL state or hand-roll a simple hash parser
- Build script approach — build.rs vs separate binary crate for YAML→JSON

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Technology Stack
- `.planning/research/STACK.md` — Leptos 0.8.x, trunk, serde-saphyr (build), serde_json (runtime), leptos-use, leptos_router
- `.planning/research/PITFALLS.md` — WASM bundle size (Pitfall 4), GitHub Pages routing (Pitfall 2), Map/WASM DOM conflict (Pitfall 7)

### Data Contract
- `pipeline/models.py` — SchoolRecord Pydantic schema defines all fields available in YAML
- `data/schools/01P03.yaml` — Example school file showing actual data structure and field names

### Requirements
- `.planning/REQUIREMENTS.md` — LIST-01..07, DETL-01..06, DEPL-01..03 define exact Phase 2 deliverables

### Architecture
- `.planning/research/ARCHITECTURE.md` — YAML schema, build step data flow
- `.planning/research/FEATURES.md` — Berlin-specific context (grundständig, admission flow, Anmeldezeitraum)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `data/schools/*.yaml` — 106 school YAML files ready for consumption
- `data/schools_index.yaml` — Flat list of all schools with basic fields (can be used for lightweight index)
- `pipeline/models.py` — SchoolRecord Pydantic model defines the canonical schema

### Established Patterns
- None — Phase 2 is the first frontend phase; establishes all Rust/Leptos patterns

### Integration Points
- Build step reads `data/schools/*.yaml` and produces embedded JSON for the WASM binary
- GitHub Actions pipeline must run trunk build and deploy dist/ to gh-pages
- Hash-based routes must be stable — Phase 3 (map) and Phase 5 (sharing) depend on URL structure

</code_context>

<specifics>
## Specific Ideas

- Reference site (lankwitz-gymnasium.lovable.app) is a single-school site — our site is multi-school comparison, different UX needs
- Profile badges should use distinct colors per type (MINT=green, music=purple, sports=blue, bilingual=orange, altsprachlich=red)
- Grundständig flag should be visually prominent — this is a primary filter for parents of 4th graders
- Detail page should feel like a "school profile card" — parents will screenshot and share with partners

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 02-spa-foundation-and-school-directory*
*Context gathered: 2026-03-26*
