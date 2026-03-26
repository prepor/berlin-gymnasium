---
phase: 01-data-pipeline
plan: "04"
subsystem: data-pipeline
tags: [python, pydantic, ruamel-yaml, deepdiff, validation, merge, changelog, conflicts]

# Dependency graph
requires:
  - phase: 01-data-pipeline-01
    provides: pipeline/seed.py — structured WFS/XLSX data as list of dicts
  - phase: 01-data-pipeline-02
    provides: pipeline/models.py — SchoolRecord Pydantic schema
  - phase: 01-data-pipeline-03
    provides: pipeline/agent.py — agent-enriched data as list of dicts
provides:
  - pipeline/validate.py — merge structured + agent data with D-17 structured-wins rule, Pydantic validation
  - pipeline/writer.py — write school YAMLs, append field-level changelog, regenerate conflicts.md
affects:
  - 01-05
  - pipeline/pipeline.py (orchestrator that will wire seed → agent → validate → write)

# Tech tracking
tech-stack:
  added: [deepdiff, ruamel.yaml (already in pyproject.toml)]
  patterns:
    - Structured-source-wins merge (D-17): structured dict baseline, agent fields only fill gaps
    - Pydantic model_validate before write (DATA-08): ValidationError skips school, logs, continues
    - ruamel.yaml round-trip write preserving Unicode and formatting
    - deepdiff with exclude_paths for changelog excluding ephemeral fields (last_updated, completeness_score)
    - _pinned_fields list in existing YAML prevents any pipeline overwrite (Pitfall 8)
    - conflicts.md regenerated (overwritten) each run; CHANGELOG.md appended (never overwritten)

key-files:
  created:
    - pipeline/validate.py
    - pipeline/writer.py
  modified: []

key-decisions:
  - "validate.py: merge_and_validate returns (SchoolRecord | None, list[dict]) — None on ValidationError, not exception propagation"
  - "writer.py: write_school_yaml returns (changed: bool, changelog_entry: str) to let write_all aggregate stats"
  - "Changelog excludes last_updated and completeness_score from deepdiff (always change, not meaningful)"
  - "write_conflicts is a separate function from write_all — caller controls when to write conflicts.md"

patterns-established:
  - "Merge pattern: start from structured dict, iterate agent fields, apply D-17/D-18/D-19 rules"
  - "YAML write pattern: read existing → apply pinned → diff → dump with ruamel.yaml"
  - "Changelog pattern: deepdiff view=text, exclude ephemeral paths, append with UTC timestamp header"

requirements-completed: [DATA-05, DATA-06, DATA-08]

# Metrics
duration: 2min
completed: 2026-03-26
---

# Phase 01 Plan 04: Validate + Writer Summary

**Merge-and-validate module (D-17 structured-wins + Pydantic) and YAML writer with deepdiff changelog and conflicts.md regeneration**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-26T14:57:29Z
- **Completed:** 2026-03-26T14:59:25Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- `pipeline/validate.py` implements structured-source-wins merge (D-17), unverified_fields tracking (D-19), conflict collection (D-20), and Pydantic validation before write (DATA-08)
- `pipeline/writer.py` writes per-school YAML with ruamel.yaml, respects `_pinned_fields` (Pitfall 8), appends field-level changelog to `data/CHANGELOG.md` via deepdiff (DATA-06), regenerates `data/conflicts.md` each run (D-20)
- Both modules expose clean public APIs (`merge_all`, `write_all`, `write_conflicts`) ready for orchestration in plan 05

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement pipeline/validate.py** - `5cb935b` (feat)
2. **Task 2: Implement pipeline/writer.py** - `f91dadd` (feat)

## Files Created/Modified

- `pipeline/validate.py` — Merge structured + agent data, Pydantic validation, conflict detection
- `pipeline/writer.py` — YAML write with pinned fields protection, deepdiff changelog, conflicts.md

## Decisions Made

- `merge_and_validate` returns `(SchoolRecord | None, list[dict])` — None on ValidationError so orchestrator can count failures without halting
- `write_school_yaml` returns `(changed: bool, changelog_entry: str)` enabling `write_all` to aggregate stats and decide whether to append to CHANGELOG.md
- deepdiff excludes `last_updated` and `completeness_score` from changelog comparison — these change on every run and would produce noise with no signal
- `write_conflicts` is intentionally separate from `write_all` so the orchestrator can pass the aggregated conflict list after processing all schools

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- `pipeline/validate.py` and `pipeline/writer.py` are ready to be imported by the orchestrator in plan 05
- `merge_all(structured_list, agent_list)` and `write_all(records, data_dir)` and `write_conflicts(all_conflicts, data_dir)` are the three public entry points
- `data/schools/` directory will be created on first write (mkdir parents=True)
- `data/CHANGELOG.md` will be created/appended on first write with changelog entries

---
*Phase: 01-data-pipeline*
*Completed: 2026-03-26*
