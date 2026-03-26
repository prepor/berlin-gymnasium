# Phase 1: Data Pipeline - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Reproducible Python pipeline producing a validated YAML corpus of all ~90 Berlin Gymnasien. Seeds from Berlin Open Data WFS, scrapes structured sources, enriches with Claude AI agents, validates against schema, writes YAML files with provenance, and produces field-level changelog on re-runs.

</domain>

<decisions>
## Implementation Decisions

### Agent Strategy
- **D-01:** Batched agents — one Claude agent handles 5-10 schools at a time to reduce API calls
- **D-02:** 8-10 parallel batches for ~5-10 min total runtime across ~90 schools
- **D-03:** Full agent autonomy — agents decide what sources to check per school
- **D-04:** Hybrid output — agent researches freely, then formats to schema-conforming JSON in same call
- **D-05:** Best-effort cross-referencing — agent tries to verify against official sources but moves on if cross-reference fails
- **D-06:** Retry with different search queries before accepting sparse data for any school
- **D-07:** Model: Sonnet for research agents (quality/cost balance)
- **D-08:** Browser tool if available for JS-rendered school websites, HTTP fetch as fallback
- **D-09:** Configurable caching — cache by school_id, skip already-researched schools unless `--force` flag is passed
- **D-10:** Detailed research log per batch for auditability
- **D-11:** Retry with exponential backoff (3 attempts) on API errors, then skip and flag school
- **D-12:** English prompts — agents search in German but report in English
- **D-13:** Budget: under $20 for a full pipeline run
- **D-14:** Agents also collect image URLs and social media links (Instagram, Facebook) for each school
- **D-15:** Quality tracking: both data completeness score (% fields filled) and per-field confidence (high/medium/low)
- **D-16:** Give agents the seed URL from WFS data, but agents can also search for alternative/updated URLs

### Conflict Resolution
- **D-17:** Structured source (WFS/schulverzeichnis) always wins when disagreeing with agent data
- **D-18:** Missing fields set to null with a note — explicit "we looked and didn't find it"
- **D-19:** Agent-only data (not in structured sources) accepted but flagged as 'unverified' in YAML
- **D-20:** Pipeline produces a separate conflicts.md showing every disagreement between sources

### Rating Sources
- **D-21:** Target specific pre-defined sites for ratings (not cast a wide net)
- **D-22:** Run an extensive research agent during planning/research phase to discover which rating sites exist for Berlin schools, then use that curated list
- **D-23:** Where ratings unavailable, use proxy signals — student-teacher ratio, Abitur averages
- **D-24:** Scrape Abitur averages from Tagesspiegel into YAML as proxy quality signal

### Pipeline Invocation
- **D-25:** Use justfile for pipeline task management (just seed, just scrape, just all, etc.)
- **D-26:** Config file (pipeline.yaml) committed to repo for defaults (minus secrets)
- **D-27:** Secrets via environment variables only (no .env file)
- **D-28:** Minimal stdout summary after each run (no separate report.md)
- **D-29:** No dry-run mode — changelog shows what changed

### Claude's Discretion
- Per-field confidence representation format (D-02 from Conflict Resolution)
- Raw anthropic SDK vs Claude Agent SDK — pick what fits the batch pipeline best
- Whether pipeline steps are independently runnable or always full end-to-end
- Manual override/edit preservation strategy for re-runs

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Data Sources
- `.planning/research/ARCHITECTURE.md` — YAML schema contract, pipeline component boundaries, data flow, concurrency model
- `.planning/research/STACK.md` — Python stack (httpx, BeautifulSoup, anthropic SDK, pydantic, ruamel.yaml), Berlin Open Data WFS details
- `.planning/research/FEATURES.md` — Berlin-specific context (grundständig, admission flow, Anmeldezeitraum)

### Pitfalls
- `.planning/research/PITFALLS.md` — Google Maps ToS (Pitfall 1), GDPR scope (Pitfall 10), data inconsistency (Pitfall 5), rating scale normalization (Pitfall 6), data staleness (Pitfall 11)

### Requirements
- `.planning/REQUIREMENTS.md` — DATA-01 through DATA-12 define exact pipeline deliverables

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- None — greenfield project, no existing code

### Established Patterns
- None — first phase establishes all patterns

### Integration Points
- Pipeline output (`data/schools/*.yaml`) is the contract consumed by Phase 2 (Leptos SPA)
- `data/CHANGELOG.md` is appended on each re-run
- `data/conflicts.md` is regenerated on each run

</code_context>

<specifics>
## Specific Ideas

- Architecture research proposed a specific YAML schema (see ARCHITECTURE.md) — use as starting point but refine during implementation
- jedeschule.de scraper exists as prior art (GitHub: Datenschule/jedeschule-scraper) — investigate for patterns
- Berlin Open Data WFS has GeoJSON export with school type filter — this is the canonical seed source
- Tagesspiegel interactive map has Abitur averages — scrape into YAML

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-data-pipeline*
*Context gathered: 2026-03-26*
