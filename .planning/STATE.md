# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-26)

**Core value:** Parents can quickly find and compare Berlin Gymnasien based on ratings, location, specialization, and grundständig (after-4th-grade) acceptance
**Current focus:** Phase 1 — Data Pipeline

## Current Position

Phase: 1 of 5 (Data Pipeline)
Plan: 0 of TBD in current phase
Status: Ready to plan
Last activity: 2026-03-26 — Roadmap created, phases derived from requirements

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Velocity:**
- Total plans completed: 0
- Average duration: —
- Total execution time: 0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| — | — | — | — |

**Recent Trend:**
- Last 5 plans: —
- Trend: —

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- (Pre-phase 1): Use Berlin Open Data WFS as canonical seed for school IDs and coordinates
- (Pre-phase 1): Use Anthropic SDK (raw tool use, not Agent SDK) for per-school web research agents
- (Pre-phase 1): Use Valhalla FOSSGIS for travel time routing — CORS must be verified before Phase 4 planning
- (Pre-phase 1): leptos-leaflet 0.8 compat is UNVERIFIED — must resolve before Phase 3 planning

### Pending Todos

None yet.

### Blockers/Concerns

- leptos-leaflet 0.8 compatibility: last known crates.io release targets Leptos 0.6. Resolve before Phase 3 planning. Fallback: raw wasm-bindgen Leaflet bindings (more effort).
- Valhalla FOSSGIS CORS: must make a real browser fetch() test to https://valhalla1.openstreetmap.de/sources_to_targets before Phase 4. Fallback: OpenRouteService free tier.
- Permitted rating sources: schulnoten.de / schulinfo.de ToS and API access unverified. If none are suitable, v1 ships without third-party ratings.

## Session Continuity

Last session: 2026-03-26
Stopped at: Roadmap and state initialized. Ready to plan Phase 1.
Resume file: None
