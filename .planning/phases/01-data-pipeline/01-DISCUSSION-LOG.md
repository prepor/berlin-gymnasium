# Phase 1: Data Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 01-data-pipeline
**Areas discussed:** Agent Strategy, Conflict Resolution, Rating Sources, Pipeline Invocation

---

## Agent Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| One agent per school | Each gets school name+website, researches independently | |
| One agent per category | One agent finds all open days, another finds all ratings | |
| Batched | One agent handles 5-10 schools at a time | ✓ |

**User's choice:** Batched — reduce API calls

| Option | Description | Selected |
|--------|-------------|----------|
| Web search + fetch only | | |
| Web search + fetch + follow links | | |
| Full autonomy | Agents decide what sources to check | ✓ |

**User's choice:** Full autonomy

| Option | Description | Selected |
|--------|-------------|----------|
| Accept sparse data | Mark missing as unknown | |
| Retry with different queries | Before accepting sparse | ✓ |
| Flag for manual review | | |

**User's choice:** Retry before accepting

| Option | Description | Selected |
|--------|-------------|----------|
| 3-5 parallel batches | ~15-30 min | |
| 8-10 parallel batches | ~5-10 min | ✓ |
| Claude decides | | |

**User's choice:** 8-10 parallel (faster, higher cost acceptable)

| Option | Description | Selected |
|--------|-------------|----------|
| Structured output | Schema-conforming JSON directly | |
| Free-form research | Prose, separate parsing step | |
| Hybrid | Research freely, format in same call | ✓ |

**User's choice:** Hybrid approach

| Option | Description | Selected |
|--------|-------------|----------|
| Must cross-reference | At least one official source | |
| Best effort | Try but move on | ✓ |
| Separate step | Validation elsewhere | |

**User's choice:** Best effort cross-referencing

**Additional decisions:**
- Model: Sonnet (quality/cost balance)
- Browser tool if available for JS-rendered sites
- Configurable cache with --force flag
- Detailed research log per batch
- Exponential backoff retry (3 attempts), then skip
- English prompts, German search
- Budget: under $20 per full run
- Collect image URLs and social media links
- Track completeness score + per-field confidence
- Give seed URL but agents can search for alternatives

---

## Conflict Resolution

| Option | Description | Selected |
|--------|-------------|----------|
| Structured always wins | Official data authoritative | ✓ |
| Newer data wins | Prefer more recent value | |
| Flag conflict | Keep both, human review | |

**User's choice:** Structured source always wins

| Option | Description | Selected |
|--------|-------------|----------|
| Omit field | Absence means unknown | |
| Set to null with note | Explicit "we looked" | ✓ |

**User's choice:** Null with note — explicit missing data representation

**Additional decisions:**
- Agent-only data accepted but flagged as 'unverified'
- Separate conflicts.md showing all disagreements
- Override/edit preservation: Claude's discretion

---

## Rating Sources

| Option | Description | Selected |
|--------|-------------|----------|
| Cast wide net | Agents search for any rating site | |
| Target specific sites | Pre-define permitted sources | ✓ |
| Claude decides | | |

**User's choice:** Target specific sites, but first run research agent to discover what exists

| Option | Description | Selected |
|--------|-------------|----------|
| No ratings available | Honest fallback | |
| Proxy signals | Student-teacher ratio, Abitur averages | |
| Both | Ratings where available, proxies everywhere | ✓ |

**User's choice:** Both ratings and proxy signals

**Additional decisions:**
- Run extensive research agent during planning to discover available rating sites
- Scrape Abitur averages from Tagesspiegel as proxy signal

---

## Pipeline Invocation

| Option | Description | Selected |
|--------|-------------|----------|
| Single script | python pipeline/run.py | |
| CLI subcommands | python -m pipeline seed/scrape/etc | |
| justfile | just seed, just scrape, just all | ✓ |

**User's choice:** justfile

| Option | Description | Selected |
|--------|-------------|----------|
| Config file | pipeline.yaml committed | ✓ |
| CLI flags only | | |
| Both | Config + CLI override | |

**User's choice:** Config file for defaults (minus secrets)

**Additional decisions:**
- Secrets via environment variables only
- Minimal stdout summary (no report.md)
- No dry-run mode
- Pipeline step independence: Claude's discretion

---

## Claude's Discretion

- Per-field confidence format
- Anthropic SDK choice (raw vs Agent SDK)
- Pipeline step independence
- Manual override/edit preservation strategy

## Deferred Ideas

None
