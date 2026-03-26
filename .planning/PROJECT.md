# Berlin Gymnasien

## What This Is

A comprehensive comparison website for all Berlin Gymnasien (~90+), helping parents choose the right secondary school for their child. The project has two parts: a reproducible data scraping pipeline that collects and validates school data from multiple sources, and a static SPA built with Rust/Leptos deployed to GitHub Pages.

## Core Value

Parents can quickly find and compare Berlin Gymnasien based on the criteria that matter most to them — ratings, location, specialization, and whether the school accepts students after 4th grade.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Scrape and validate data for all Berlin Gymnasien
- [ ] One YAML file per school with structured data
- [ ] Collect ratings from as many sources as possible
- [ ] Flag which schools accept students after 4th grade
- [ ] Reproducible scraping pipeline with changelog on re-run
- [ ] Document how data was gathered
- [ ] Static SPA with school listing and filters
- [ ] Interactive map showing school locations
- [ ] Travel time estimation (foot, public transport, car) from user-entered address
- [ ] Comparison mode (2-3 schools side by side)
- [ ] Deploy to GitHub Pages

### Out of Scope

- Dynamic backend / database — static site only
- User accounts / login — no server-side state
- Real-time data updates — manual re-run of pipeline
- Mobile native app — responsive web only

## Context

**Domain:** Berlin has ~90+ Gymnasien across 12 districts. Most accept students after 6th grade (standard), but ~30 accept after 4th grade — a key decision point for parents. School quality varies significantly and is hard to assess from official data alone.

**Data sources (to discover):**
- Official: schulverzeichnis.berlin.de, school websites
- Ratings: Google Maps, various school comparison/ranking sites
- Structured: Berlin open data, school directories

**Data points per school:**
- Basic: name, address, phone, email, website
- Numbers: student count, teacher count, student-teacher ratio
- Admission: accepts after 4th grade (yes/no), admission criteria, Notendurchschnitt, oversubscription ratio
- Profile: specialization (MINT/music/sports/bilingual/altsprachlich), Ganztag vs half-day
- Languages: which foreign languages offered, from which grade
- Facilities: cafeteria, accessibility/barrier-free
- Programs: IB, honors tracks, exchange programs
- Events: next open day (Tag der offenen Tür)
- Ratings: aggregated from multiple sources with source attribution
- Location: coordinates (lat/lng), district (Bezirk)

**Scraping approach:** Mix of Claude agents (web research, discovery, validation) and scripts (structured API calls, data extraction). Run in parallel where possible.

**Reference:** https://lankwitz-gymnasium.lovable.app/ — single-school example site

## Constraints

- **Tech stack (site)**: Rust + Leptos — static SPA compiled to WASM
- **Deployment**: GitHub Pages — no server-side rendering or API
- **Data format**: YAML files, one per school, in `data/schools/`
- **Travel time API**: Needs a free/affordable routing API (likely OSRM or similar) — called client-side
- **Scraping**: Must be reproducible and produce a changelog showing what changed between runs

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + Leptos for frontend | User preference, WASM-based SPA | — Pending |
| One YAML file per school | Clean organization, easy to diff/review | — Pending |
| GitHub Pages deployment | Free, simple, no backend needed | — Pending |
| Client-side travel time | No backend, use OSRM or similar from browser | — Pending |
| Mix of agents + scripts for scraping | Agents for discovery/validation, scripts for structured sources | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd:transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-26 after initialization*
