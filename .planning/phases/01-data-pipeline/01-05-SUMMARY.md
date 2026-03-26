---
phase: 01-data-pipeline
plan: "05"
subsystem: data-pipeline
tags: [python, asyncio, argparse, ruamel-yaml, pipeline-orchestrator, readme]

# Dependency graph
requires:
  - phase: 01-data-pipeline-01
    provides: justfile, pipeline.yaml, pyproject.toml — task runner and config
  - phase: 01-data-pipeline-02
    provides: pipeline/seed.py — run_seed(config) public interface
  - phase: 01-data-pipeline-03
    provides: pipeline/agent.py — run_enrich(schools, config, force) public interface
  - phase: 01-data-pipeline-04
    provides: pipeline/validate.py — merge_all(structured, agent); pipeline/writer.py — write_all, write_conflicts
provides:
  - pipeline/run.py — pipeline orchestrator with argument parsing, step dispatch, config loading, ANTHROPIC_API_KEY fail-fast
  - README.md — setup and usage documentation covering prerequisites, setup, pipeline steps, configuration, data output
affects:
  - All future consumers of the pipeline (just all, just seed, etc.)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Lazy imports inside step functions — modules loaded only when step is invoked (avoids import errors from missing deps)"
    - "check_api_key(step) fail-fast pattern — sys.exit(1) with clear message before any work starts"
    - "load_config() tries both the given path and pipeline/path for flexible invocation from workspace root"
    - "load_enriched_cache() reads pipeline/cache/*.json to support standalone validate and write steps"

key-files:
  created:
    - pipeline/run.py
    - README.md
  modified: []

key-decisions:
  - "Step imports are lazy (inside functions) so run.py can be imported without installing pipeline deps"
  - "ANTHROPIC_API_KEY check uses sys.exit(1) (not EnvironmentError) for clean CLI exit with user-readable message"
  - "Scrape step is a no-op with an informational print — WFS covers all structured fields (no separate scrape needed)"
  - "validate and write standalone steps load from cache file so they work without running enrich first"

patterns-established:
  - "Pattern: orchestrator dispatches via lazy imports — `from pipeline.module import func` inside the step function"
  - "Pattern: run_all() is a separate coroutine from main() for testability"

requirements-completed: [DATA-01, DATA-02, DATA-03, DATA-04, DATA-05, DATA-06, DATA-07, DATA-08, DATA-09, DATA-10, DATA-11, DATA-12]

# Metrics
duration: 5min
completed: 2026-03-26
---

# Phase 01 Plan 05: Pipeline Orchestrator + README Summary

**pipeline/run.py orchestrator wiring seed → enrich → validate → write with ANTHROPIC_API_KEY fail-fast, and README.md covering full setup and usage documentation**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-26T15:02:46Z
- **Completed:** 2026-03-26T15:07:45Z
- **Tasks:** 2 (1 auto + 1 checkpoint)
- **Files modified:** 2

## Accomplishments

- `pipeline/run.py` wires all 5 step modules via `--step` dispatch: seed, scrape (no-op), enrich, validate, write, all
- ANTHROPIC_API_KEY fail-fast: `sys.exit(1)` with clear message before any work for `enrich` and `all` steps
- Config loaded from `pipeline/pipeline.yaml` via ruamel.yaml; logging configured from `config.log_level`
- `just all` dispatches through `run_all()`: seed → scrape → enrich → validate+merge → write with step progress output
- `README.md` covers all required sections: Prerequisites, Setup, Running the Pipeline, Pipeline Steps, Configuration, Data Output

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement pipeline/run.py** - `843f5d6` (feat)
2. **Task 2: Write README.md (checkpoint pre-action)** - `23cf3f9` (docs)

## Files Created/Modified

- `pipeline/run.py` — Pipeline orchestrator: argument parsing, step dispatch, config loading, API key guard
- `README.md` — Setup and usage documentation with all required sections

## Decisions Made

- Step functions use lazy imports (e.g., `from pipeline.seed import run_seed` inside the function body) so `run.py` can be invoked without all pipeline modules being importable at import time
- `check_api_key()` calls `sys.exit(1)` rather than raising `EnvironmentError` — produces clean CLI output without a Python traceback for users who haven't set the key
- Standalone `validate` and `write` steps call `load_enriched_cache()` to read from `pipeline/cache/*.json` so these steps can be run independently after enrichment
- `scrape` step is a deliberate no-op with an informational message — WFS provides all structured fields, no additional scraping is needed at this time

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — pipeline/run.py requires no external services beyond what was already required. ANTHROPIC_API_KEY for enrich step was documented in README.md setup section.

## Next Phase Readiness

- Complete pipeline scaffold is ready: `just seed` produces `data/schools_index.yaml`; `just all` runs the full pipeline end-to-end
- All DATA requirements (DATA-01 through DATA-12) are satisfied by the combined plans 01-05
- Phase 01 data pipeline is complete — ready for Phase 02 (Leptos SPA)

---
*Phase: 01-data-pipeline*
*Completed: 2026-03-26*

## Self-Check: PASSED

- FOUND: pipeline/run.py
- FOUND: README.md
- FOUND: .planning/phases/01-data-pipeline/01-05-SUMMARY.md
- FOUND: commit 843f5d6 (feat: pipeline/run.py)
- FOUND: commit 23cf3f9 (docs: README.md)
