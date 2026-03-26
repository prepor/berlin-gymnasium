---
phase: 01-data-pipeline
plan: "02"
subsystem: data-pipeline
tags: [python, httpx, openpyxl, ruamel-yaml, wfs, gdi-berlin, eckdaten, seed]

# Dependency graph
requires:
  - phase: 01-data-pipeline-01
    provides: pipeline scaffold (models.py, pipeline.yaml, justfile, pyproject.toml)
provides:
  - pipeline/seed.py — async WFS fetch + Eckdaten XLSX merge producing data/schools_index.yaml
  - run_seed(config) public interface for pipeline orchestrator
affects:
  - 01-data-pipeline plans 03+ (enrich, validate, write all consume schools_index.yaml)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "WFS fetch with srsName=EPSG:4326 to get WGS84 coordinates (not EPSG:25833)"
    - "XLSX downloaded via httpx into io.BytesIO, parsed with openpyxl.load_workbook"
    - "BSN normalization: str(bsn).strip() on both WFS and XLSX sides for merge"
    - "Coordinate sanity check: raise ValueError if lat/lng > 1000 (EPSG:25833 UTM detection)"

key-files:
  created:
    - pipeline/seed.py
  modified: []

key-decisions:
  - "XLSX fetch failures are non-fatal: returns empty dict so pipeline continues with null counts"
  - "include_private_schools from pipeline.yaml controls traeger filtering (default: true)"
  - "Missing fields set to null in schools_index.yaml (intermediate file, not SchoolRecord)"

patterns-established:
  - "Pattern: seed.py is both importable (run_seed) and directly runnable (__main__)"
  - "Pattern: config dict flows from pipeline.yaml through run_seed — no global state"

requirements-completed: [DATA-01, DATA-02]

# Metrics
duration: 2min
completed: 2026-03-26
---

# Phase 01 Plan 02: Seed Step Summary

**Async WFS fetch from gdi.berlin.de/services/wfs/schulen with EPSG:4326, merged with Eckdaten XLSX student/teacher counts via openpyxl, writes data/schools_index.yaml**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-26T14:49:11Z
- **Completed:** 2026-03-26T14:50:35Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Implemented `pipeline/seed.py` with `fetch_gymnasien()`, `fetch_eckdaten()`, `merge_eckdaten()`, `write_index()`, and `run_seed()` functions
- WFS fetch enforces `srsName=EPSG:4326` to get WGS84 coordinates; raises `ValueError` if non-WGS84 coords detected
- Eckdaten XLSX downloaded via httpx, parsed with openpyxl; BSN normalized to string for merge; fetch failures are non-fatal
- `run_seed(config: dict) -> list[dict]` is the public interface for the pipeline orchestrator

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement pipeline/seed.py — WFS fetch + Eckdaten XLSX merge** - `941db02` (feat)

**Plan metadata:** _(to be committed)_

## Files Created/Modified

- `pipeline/seed.py` — Seed step: async WFS fetch, Eckdaten XLSX merge, writes data/schools_index.yaml

## Decisions Made

- XLSX fetch failures are non-fatal (returns empty dict) so pipeline continues with null student/teacher counts — allows seed to succeed when Eckdaten URL is unavailable
- `include_private_schools` config key (from pipeline.yaml, default `true`) controls whether private schools (traeger != "öffentlich") are included
- BSN columns in XLSX headers checked as "BSN", "Schulnummer", "bsn" and student/teacher columns via partial match — defensive against column name variation in future XLSX editions

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required. Network access to `gdi.berlin.de` and `www.berlin.de` needed at runtime.

## Next Phase Readiness

- `pipeline/seed.py` is ready; `just seed` will produce `data/schools_index.yaml` with all Gymnasien
- Plan 03 (scrape/enrich) can consume `schools_index.yaml` via `run_seed()` return value or by reading the YAML file
- No blockers for downstream plans

## Self-Check: PASSED

- FOUND: pipeline/seed.py
- FOUND: .planning/phases/01-data-pipeline/01-02-SUMMARY.md
- FOUND: commit 941db02

---
*Phase: 01-data-pipeline*
*Completed: 2026-03-26*
