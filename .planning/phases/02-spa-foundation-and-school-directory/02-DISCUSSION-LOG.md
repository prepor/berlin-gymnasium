# Phase 2: SPA Foundation and School Directory - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 02-spa-foundation-and-school-directory
**Areas discussed:** Visual design, Filter UI, School card content, Detail page layout, Routing approach, Language/locale
**Mode:** --auto (all recommended defaults selected)

---

## Visual Design Direction

| Option | Description | Selected |
|--------|-------------|----------|
| Clean card grid | Mobile-first cards with profile badges, modern and scannable | auto |
| Dense data table | Spreadsheet-like, high information density | |
| Hybrid list/cards | Cards on mobile, table on desktop | |

**User's choice:** [auto] Clean card grid (recommended — standard for school comparison sites)
**Notes:** Mobile-first is essential for parents browsing on phones.

---

## Filter UI Pattern

| Option | Description | Selected |
|--------|-------------|----------|
| Sidebar + collapsible mobile panel | Desktop sidebar, collapses to top panel on mobile | auto |
| Top bar filter chips | Always-visible horizontal chip strip | |
| Full-page filter overlay | Tap to open overlay, apply, close | |

**User's choice:** [auto] Sidebar + collapsible mobile panel (recommended — established pattern)
**Notes:** AND logic for filter combination.

---

## School Card Content

| Option | Description | Selected |
|--------|-------------|----------|
| Key info cards | Name, district, profile badges, grundständig flag, student count | auto |
| Minimal cards | Name and district only, click for everything else | |
| Rich preview cards | Include admission info, languages, ratings in card | |

**User's choice:** [auto] Key info cards (recommended — enough to decide whether to click)

---

## Detail Page Layout

| Option | Description | Selected |
|--------|-------------|----------|
| Sectioned profile | Hero → Profile → Admission → Ratings → Contact → Provenance | auto |
| Tabbed layout | Tabs for different data categories | |
| Single scroll | Everything in one long scroll, no sections | |

**User's choice:** [auto] Sectioned profile (recommended — scannable, printable)
**Notes:** Missing data shown as "Keine Angabe" rather than hidden.

---

## Routing Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Hash routing | /#/ and /#/school/{id} — no server config needed | auto |
| HTML5 history + 404.html | Clean URLs but needs 404.html redirect hack | |

**User's choice:** [auto] Hash routing (recommended — simplest for GitHub Pages)

---

## Language/Locale

| Option | Description | Selected |
|--------|-------------|----------|
| German only | All UI in German — target audience is Berlin parents | auto |
| English only | | |
| Bilingual toggle | | |

**User's choice:** [auto] German only (recommended — primary audience)

---

## Claude's Discretion

- CSS styling and color palette
- Component decomposition
- leptos-use vs hand-rolled URL state
- build.rs vs separate binary for YAML→JSON

## Deferred Ideas

None.
