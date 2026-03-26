---
phase: 01-data-pipeline
verified: 2026-03-26T16:15:00Z
status: gaps_found
score: 14/17 must-haves verified
re_verification: false
gaps:
  - truth: "Running `just all` completes the full pipeline: seed → scrape → enrich → validate → write (end-to-end verified)"
    status: partial
    reason: "pipeline/agent.py was committed (ff26135) and all code is present and substantive, but plan 01-03 has no SUMMARY.md and ROADMAP.md marks it as [ ] (not complete). The file exists and is wired, but the plan was never formally closed. The pipeline technically runs but this plan closure gap means phase completion is uncertain."
    artifacts:
      - path: ".planning/phases/01-data-pipeline/01-03-SUMMARY.md"
        issue: "File does not exist — plan 01-03 was never formally completed despite agent.py being committed"
    missing:
      - "Create 01-03-SUMMARY.md for the agent enrichment plan"
      - "Update ROADMAP.md to mark 01-03-PLAN.md as [x] complete"
  - truth: "data/schools_index.yaml contains all 95+ public Gymnasien (produced at runtime)"
    status: failed
    reason: "data/schools_index.yaml does not exist — the pipeline has never been run end-to-end. This is a runtime artifact, not a code artifact, but the ROADMAP Success Criterion 1 requires 'Running the pipeline produces one YAML file per school' — which cannot be confirmed without a run."
    artifacts:
      - path: "data/schools_index.yaml"
        issue: "File does not exist — pipeline has not been executed yet"
      - path: "data/schools/*.yaml"
        issue: "data/schools/ is empty (only .gitkeep) — no school YAML files have been produced"
    missing:
      - "Execute `just seed` to produce data/schools_index.yaml and verify school count is 95–110 with WGS84 coordinates"
      - "Execute `just all` with ANTHROPIC_API_KEY to produce data/schools/*.yaml files (or document as human verification)"
  - truth: "data/CHANGELOG.md is created after the first run and appended on subsequent runs"
    status: failed
    reason: "data/CHANGELOG.md does not exist — the write step has never been executed"
    artifacts:
      - path: "data/CHANGELOG.md"
        issue: "File does not exist — write step has not been run"
    missing:
      - "Execute `just all` end-to-end to produce data/CHANGELOG.md"
human_verification:
  - test: "Run `just seed` and check data/schools_index.yaml"
    expected: "95–110 school entries with lat ~52.x, lng ~13.x (WGS84), not UTM coordinates >1000"
    why_human: "Requires live network access to gdi.berlin.de WFS endpoint"
  - test: "Run `just all` with a valid ANTHROPIC_API_KEY"
    expected: "data/schools/*.yaml files created (one per school), data/CHANGELOG.md created, data/conflicts.md created"
    why_human: "Requires ANTHROPIC_API_KEY and live web search API calls; cannot be verified programmatically"
---

# Phase 1: Data Pipeline Verification Report

**Phase Goal:** A complete, validated, reproducible YAML corpus of all Berlin Gymnasien exists and can be re-run to produce a field-level changelog
**Verified:** 2026-03-26T16:15:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

The phase goal requires the corpus to EXIST, not merely for the code to exist to produce it. The pipeline code is complete and substantive, but the pipeline has never been executed — data/schools_index.yaml, data/schools/*.yaml, and data/CHANGELOG.md are all absent. Additionally, plan 01-03 (agent enrichment) was implemented but never formally closed.

### Observable Truths

| #  | Truth | Status | Evidence |
|----|-------|--------|----------|
| 1  | Single `just all` command exists and is documented | VERIFIED | justfile has 7 targets including `all` and `force`; README.md documents `just all` |
| 2  | Pydantic SchoolRecord defines all required fields (school_id, name, district, accepts_after_4th_grade, ratings, admission_requirements, open_day, provenance) | VERIFIED | pipeline/models.py lines 58–95 define all required fields with correct types |
| 3  | `uv sync` installs all pipeline dependencies | VERIFIED | uv.lock (474 lines) present with all 8 pinned deps; pyproject.toml correct |
| 4  | pipeline.yaml config exists with all 9 keys | VERIFIED | All 9 config keys present: wfs_url, eckdaten_url, include_private_schools, batch_size, semaphore_limit, max_web_searches_per_school, cache_dir, data_dir, index_file, log_level |
| 5  | Missing ANTHROPIC_API_KEY causes clear error before any work | VERIFIED | run.py check_api_key() calls sys.exit(1) with user-readable message for enrich/all steps |
| 6  | WFS fetch uses srsName=EPSG:4326 (no UTM coordinates) | VERIFIED | seed.py line 33 includes the parameter; line 150–155 validates coords are not >1000 |
| 7  | XLSX fetch uses openpyxl.load_workbook | VERIFIED | seed.py line 80 uses openpyxl.load_workbook(io.BytesIO(...)) |
| 8  | Agent uses AsyncAnthropic client (not sync + run_in_executor) | VERIFIED | agent.py lines 29–37: AsyncAnthropic client; all calls use await |
| 9  | Agent uses web_search_20260209 tool | VERIFIED | agent.py line 163: "type": "web_search_20260209" |
| 10 | Agent caches per school_id in pipeline/cache/ | VERIFIED | agent.py has read_cache/write_cache functions using Path(cache_dir) / f"{school_id}.json" |
| 11 | Tenacity retry (3 attempts, exponential backoff) on API errors | VERIFIED | agent.py lines 225–228: stop_after_attempt(3), wait_exponential(multiplier=2, min=4, max=60) |
| 12 | Structured source fields win over agent data (D-17) | VERIFIED | validate.py STRUCTURED_FIELDS set; agent fields in STRUCTURED_FIELDS are not overwritten |
| 13 | All records pass Pydantic validation before write | VERIFIED | validate.py line 95: SchoolRecord.model_validate(merged); None returned on ValidationError |
| 14 | Re-running pipeline appends to data/CHANGELOG.md (not overwrites) | VERIFIED | writer.py line 152: open("a") mode; line 176: conflicts.md uses write_text (overwrite per spec) |
| 15 | run.py wires all steps (seed, enrich, validate, write) | VERIFIED | All 4 lazy imports confirmed: pipeline.seed, pipeline.agent, pipeline.validate, pipeline.writer |
| 16 | data/schools_index.yaml produced with 95–110 schools | FAILED | File does not exist — pipeline has not been executed |
| 17 | data/CHANGELOG.md created after first run | FAILED | File does not exist — write step has not been executed |

**Score:** 14/17 truths verified (3 gaps — 1 partial plan closure, 2 runtime artifacts not yet produced)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `pipeline/models.py` | Pydantic SchoolRecord, RatingEntry, LanguageEntry, AdmissionRequirements | VERIFIED | All 4 classes present; SchoolRecord has all DATA-04/05/09/10/11/12 fields |
| `pipeline/pyproject.toml` | uv-managed project with 8 pinned deps including anthropic==0.86.0 | VERIFIED | All 8 dependencies at pinned versions; hatchling build config present |
| `pipeline/pipeline.yaml` | Config with 9 keys including semaphore_limit | VERIFIED | All 9 keys present with correct defaults |
| `justfile` | 7 targets: seed, scrape, enrich, validate, write, all, force | VERIFIED | All 7 targets present using `uv run --project pipeline` pattern |
| `pipeline/seed.py` | WFS fetch + XLSX merge → schools_index.yaml | VERIFIED | 184 lines; fetch_gymnasien, fetch_eckdaten, merge_eckdaten, run_seed all implemented |
| `pipeline/agent.py` | Claude enrichment with web_search_20260209 | VERIFIED | 309 lines; enrich_batch, run_enrich, caching, retry all implemented |
| `pipeline/validate.py` | Merge + Pydantic validation with conflict detection | VERIFIED | merge_and_validate, merge_all, STRUCTURED_FIELDS, unverified_fields tracking |
| `pipeline/writer.py` | YAML write + deepdiff changelog + conflicts.md | VERIFIED | write_all, write_conflicts, apply_pinned_fields, DeepDiff present |
| `pipeline/run.py` | Orchestrator wiring all steps | VERIFIED | All 4 lazy imports present; check_api_key with sys.exit(1) |
| `README.md` | Setup and usage documentation | VERIFIED | All 6 required sections present: Prerequisites, Setup, Running the Pipeline, Pipeline Steps, Configuration, Data Output |
| `pipeline/uv.lock` | Dependency lock file | VERIFIED | 474-line lock file present with all 8 deps at pinned versions |
| `.gitignore` | Python/cache exclusions | VERIFIED | .venv/, pipeline/cache/*.json, *.pyc, .env present; data/schools/ NOT excluded (correct) |
| `data/schools_index.yaml` | Runtime artifact: 95–110 school entries | MISSING | Pipeline not yet executed |
| `data/schools/*.yaml` | Runtime artifact: one YAML per school | MISSING | Pipeline not yet executed; only .gitkeep present |
| `data/CHANGELOG.md` | Runtime artifact: field-level changelog | MISSING | Pipeline not yet executed |
| `.planning/phases/01-data-pipeline/01-03-SUMMARY.md` | Plan closure document for agent plan | MISSING | agent.py was committed (ff26135) but plan was never formally closed |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `justfile` | `pipeline/run.py` | `uv run --project pipeline python pipeline/run.py --step {step}` | VERIFIED | All 7 targets use this pattern (line 8 in justfile) |
| `pipeline/models.py` | All pipeline modules | `from pipeline.models import SchoolRecord` | VERIFIED | validate.py line 17, writer.py line 19 import SchoolRecord |
| `pipeline/run.py` | `pipeline/seed.py` | `from pipeline.seed import run_seed` | VERIFIED | run.py line 48 (lazy import in step_seed) |
| `pipeline/run.py` | `pipeline/agent.py` | `from pipeline.agent import run_enrich` | VERIFIED | run.py line 59 (lazy import in step_enrich) |
| `pipeline/run.py` | `pipeline/validate.py` | `from pipeline.validate import merge_all` | VERIFIED | run.py line 67 (lazy import in step_validate_and_merge) |
| `pipeline/run.py` | `pipeline/writer.py` | `from pipeline.writer import write_all, write_conflicts` | VERIFIED | run.py line 72 (lazy import in step_write) |
| `pipeline/seed.py` | WFS endpoint | `httpx.AsyncClient GET with srsName=EPSG:4326` | VERIFIED | seed.py lines 27–34: params dict with srsName=EPSG:4326 |
| `pipeline/seed.py` | Eckdaten XLSX | `httpx + openpyxl.load_workbook(io.BytesIO(...))` | VERIFIED | seed.py lines 72–80 |
| `pipeline/agent.py` | `anthropic.AsyncAnthropic` | `client.messages.create with tools=[{type: web_search_20260209}]` | VERIFIED | agent.py lines 158–173 |
| `pipeline/agent.py` | `pipeline/cache/{school_id}.json` | `json.dump / json.load for per-school result caching` | VERIFIED | agent.py cache_path, read_cache, write_cache functions |
| `pipeline/writer.py` | `data/CHANGELOG.md` | `deepdiff DeepDiff + append with open("a")` | VERIFIED | writer.py lines 148–153: open("a") append mode |
| `pipeline/writer.py` | `data/schools/{school_id}.yaml` | `ruamel.yaml YAML().dump(record_dict, file)` | VERIFIED | writer.py lines 119–121 |

### Data-Flow Trace (Level 4)

This is a data pipeline, not a UI component. Data flows at runtime through external APIs (WFS, XLSX, Claude). Static code analysis cannot trace live API responses. Key runtime flow is:

1. seed.py → WFS GeoJSON → schools list (runtime network call)
2. seed.py → Eckdaten XLSX → student/teacher counts merged by BSN (runtime network call)
3. agent.py → Claude API + web_search tool → enriched JSON per school (runtime API call)
4. validate.py → merged dict → SchoolRecord validation (deterministic, verifiable)
5. writer.py → SchoolRecord dict → YAML file + CHANGELOG.md append (deterministic, verifiable)

Code structure for steps 4–5 is substantive and correct. Steps 1–3 require runtime execution.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| All 6 Python files parse cleanly | `python3 -c "import ast; [ast.parse(open(f).read()) for f in [...]]"` | "All 6 files: syntax OK" | PASS |
| uv.lock present with all 8 deps | Check uv.lock for all 8 dependency names | All 8 names found in uv.lock at correct versions | PASS |
| run.py has all 4 step imports | grep for `from pipeline.seed/agent/validate/writer` | All 4 FOUND | PASS |
| `just seed` produces schools_index.yaml | Network call required | SKIP — requires network to gdi.berlin.de | SKIP |
| `just all` produces school YAML files | Requires ANTHROPIC_API_KEY | SKIP — requires API key + network | SKIP |
| SchoolRecord imports correctly | Requires uv/Python 3.13 with deps installed | uv not available in verification env; uv.lock confirms deps are locked | SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| DATA-01 | 01-02, 01-05 | Pipeline seeds canonical list from WFS with school IDs, coordinates, district | SATISFIED | seed.py fetches WFS with EPSG:4326; run.py dispatches seed step |
| DATA-02 | 01-02, 01-05 | Scrapes structured data (address, phone, website, student/teacher count) | SATISFIED | seed.py extracts address, phone, website, traeger from WFS; XLSX merge adds student/teacher counts |
| DATA-03 | 01-03, 01-05 | Claude agents enrich each school with profile, languages, Ganztag, open day, ratings | SATISFIED (code) | agent.py implements batched Claude enrichment with all DATA-03 fields in prompt; plan closure gap noted |
| DATA-04 | 01-01, 01-04, 01-05 | Each school stored as one YAML file in data/schools/{school_id}.yaml | SATISFIED (code) | writer.py line 101: `Path(data_dir) / f"{record.school_id}.yaml"`; not yet produced at runtime |
| DATA-05 | 01-01, 01-04 | YAML schema includes provenance tracking (data_sources, last_updated per school) | SATISFIED | models.py has data_sources, last_updated, unverified_fields; validate.py populates them |
| DATA-06 | 01-04 | Pipeline re-run produces field-level changelog in data/CHANGELOG.md | SATISFIED (code) | writer.py uses DeepDiff + open("a") append; CHANGELOG.md not yet produced at runtime |
| DATA-07 | 01-01, 01-05 | Pipeline runs reproducibly with documented setup and single command execution | SATISFIED | README.md documents all setup steps; `just all` is the single command |
| DATA-08 | 01-01, 01-04 | Pipeline validates all school records against Pydantic schema before writing | SATISFIED | validate.py SchoolRecord.model_validate() before any write; ValidationError skips school |
| DATA-09 | 01-01, 01-03 | Collects ratings from multiple permitted sources (not Google Maps) with source attribution | SATISFIED (code) | agent.py prompt explicitly excludes Google Maps; RatingEntry has source, score, scale, review_count |
| DATA-10 | 01-01, 01-03 | Flags which schools accept students after 4th grade (grundständig) | SATISFIED (code) | SchoolRecord.accepts_after_4th_grade field; agent prompt searches for grundständig flag |
| DATA-11 | 01-01, 01-03 | Collects admission requirements per school | SATISFIED (code) | AdmissionRequirements model; agent prompt requests notendurchschnitt, oversubscribed, probeunterricht, entrance_test |
| DATA-12 | 01-01, 01-03 | Collects open day (Tag der offenen Tür) dates | SATISFIED (code) | SchoolRecord.open_day field; agent prompt searches for "Tag der offenen Tür 2026" |

**Orphaned requirements check:** All 12 DATA requirements (DATA-01 through DATA-12) are claimed by at least one plan. No orphaned requirements.

### Anti-Patterns Found

| File | Pattern | Severity | Impact |
|------|---------|----------|--------|
| `pipeline/run.py` line 52–55 | `step_scrape` is a deliberate no-op (`print` only) | Info | Expected and documented in ROADMAP/README as "WFS covers all structured fields" — not a stub, by design |
| `pipeline/agent.py` lines 184–190 | `return []` on JSON parse failure | Info | Proper error handling, not a stub; logged with warning |

No blockers found. No stub implementations. All `return {}` / `return []` patterns are defensive fallbacks in error paths, not hollow implementations.

### Human Verification Required

#### 1. Seed Step End-to-End

**Test:** Run `just seed` from the workspace root (requires network access to gdi.berlin.de and berlin.de)
**Expected:** data/schools_index.yaml created with 95–110 entries; all `coords.lat` values ~52.x and `coords.lng` values ~13.x (WGS84, not UTM >1000)
**Verification command:** `python3 -c "from ruamel.yaml import YAML; y=YAML(); d=y.load(open('data/schools_index.yaml')); bad=[s for s in d if s['coords']['lat']>1000]; print(f'{len(d)} schools, {len(bad)} bad coords')"`
**Why human:** Requires live network call to official Berlin Open Data WFS service

#### 2. Full Pipeline End-to-End

**Test:** Run `just all` with a valid ANTHROPIC_API_KEY set
**Expected:** data/schools/*.yaml files created (one per school, ~95–110 files); data/CHANGELOG.md created with at least one run entry; data/conflicts.md created; all YAML files contain school_id, name, district, last_updated at minimum
**Why human:** Requires ANTHROPIC_API_KEY and live Claude API + web search calls (~5–15 minute run time)

#### 3. Plan 01-03 Formal Closure

**Test:** Review that 01-03-SUMMARY.md exists and ROADMAP.md marks 01-03-PLAN.md as [x]
**Expected:** 01-03-SUMMARY.md present; ROADMAP.md line `- [x] 01-03-PLAN.md — Agent enrichment`
**Why human:** This is a documentation closure task, not a code verification

### Gaps Summary

Three gaps block full phase completion:

**Gap 1 (Plan closure):** Plan 01-03 (agent enrichment) was implemented in commit `ff26135` but the plan was never formally closed. pipeline/agent.py is substantive and fully wired, but 01-03-SUMMARY.md is absent and ROADMAP.md marks the plan as incomplete. This is a documentation/process gap, not a code gap.

**Gap 2–3 (Runtime artifacts):** The pipeline code is complete and correct, but the pipeline has never been executed. The phase goal requires the corpus to exist ("a complete, validated, reproducible YAML corpus of all Berlin Gymnasien EXISTS"), and data/schools_index.yaml, data/schools/*.yaml, and data/CHANGELOG.md are all absent. These require either human execution or a CI/CD run to produce.

The code path from `just seed` → schools_index.yaml → agent enrichment → Pydantic validation → YAML write → CHANGELOG.md is fully wired and substantive. Running the pipeline should produce all expected artifacts. The gaps are execution gaps, not implementation gaps.

---

_Verified: 2026-03-26T16:15:00Z_
_Verifier: Claude (gsd-verifier)_
