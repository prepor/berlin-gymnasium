# Phase 1: Data Pipeline - Research

**Researched:** 2026-03-26
**Domain:** Python data pipeline — WFS seed, structured scraping, Claude AI enrichment, Pydantic validation, YAML corpus
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Agent Strategy**
- D-01: Batched agents — one Claude agent handles 5-10 schools at a time to reduce API calls
- D-02: 8-10 parallel batches for ~5-10 min total runtime across ~90 schools
- D-03: Full agent autonomy — agents decide what sources to check per school
- D-04: Hybrid output — agent researches freely, then formats to schema-conforming JSON in same call
- D-05: Best-effort cross-referencing — agent tries to verify against official sources but moves on if cross-reference fails
- D-06: Retry with different search queries before accepting sparse data for any school
- D-07: Model: claude-sonnet-4-6 for research agents (quality/cost balance)
- D-08: Browser tool if available for JS-rendered school websites, HTTP fetch as fallback
- D-09: Configurable caching — cache by school_id, skip already-researched schools unless `--force` flag is passed
- D-10: Detailed research log per batch for auditability
- D-11: Retry with exponential backoff (3 attempts) on API errors, then skip and flag school
- D-12: English prompts — agents search in German but report in English
- D-13: Budget: under $20 for a full pipeline run
- D-14: Agents also collect image URLs and social media links (Instagram, Facebook) for each school
- D-15: Quality tracking: both data completeness score (% fields filled) and per-field confidence (high/medium/low)
- D-16: Give agents the seed URL from WFS data, but agents can also search for alternative/updated URLs

**Conflict Resolution**
- D-17: Structured source (WFS/schulverzeichnis) always wins when disagreeing with agent data
- D-18: Missing fields set to null with a note — explicit "we looked and didn't find it"
- D-19: Agent-only data (not in structured sources) accepted but flagged as 'unverified' in YAML
- D-20: Pipeline produces a separate conflicts.md showing every disagreement between sources

**Rating Sources**
- D-21: Target specific pre-defined sites for ratings (not cast a wide net)
- D-22: Run an extensive research agent during planning/research phase to discover which rating sites exist for Berlin schools, then use that curated list
- D-23: Where ratings unavailable, use proxy signals — student-teacher ratio, Abitur averages
- D-24: Scrape Abitur averages from Tagesspiegel into YAML as proxy quality signal

**Pipeline Invocation**
- D-25: Use justfile for pipeline task management (just seed, just scrape, just all, etc.)
- D-26: Config file (pipeline.yaml) committed to repo for defaults (minus secrets)
- D-27: Secrets via environment variables only (no .env file)
- D-28: Minimal stdout summary after each run (no separate report.md)
- D-29: No dry-run mode — changelog shows what changed

### Claude's Discretion
- Per-field confidence representation format
- Raw anthropic SDK vs Claude Agent SDK — pick what fits the batch pipeline best
- Whether pipeline steps are independently runnable or always full end-to-end
- Manual override/edit preservation strategy for re-runs

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DATA-01 | Pipeline seeds canonical list of all Berlin Gymnasien from Berlin Open Data WFS with school IDs, coordinates, and district | WFS endpoint confirmed at gdi.berlin.de; 95 public + 11 private Gymnasien; WGS84 output via srsName=EPSG:4326 confirmed |
| DATA-02 | Pipeline scrapes structured data (address, phone, website, student count, teacher count) from official sources | WFS provides address, phone, website fields directly; Eckdaten XLSX at berlin.de provides student/teacher counts |
| DATA-03 | Claude AI agents enrich each school with profile/specialization, languages offered, Ganztag status, open day dates, and ratings from discoverable sources | anthropic SDK 0.86.0 with web_search_20260209 server tool confirmed; claude-sonnet-4-6 supports it |
| DATA-04 | Each school stored as one YAML file in data/schools/{school_id}.yaml with defined schema | ruamel.yaml 0.19.1 for round-trip writes; YAML schema contract in ARCHITECTURE.md |
| DATA-05 | YAML schema includes provenance tracking (data_sources, last_updated per school) | data_sources list + last_updated field in schema; per-field confidence from D-15 |
| DATA-06 | Pipeline re-run produces field-level changelog (data/CHANGELOG.md) showing what changed | deepdiff 8.6.2 for nested dict diff; append-mode changelog pattern from ARCHITECTURE.md |
| DATA-07 | Pipeline runs reproducibly with documented setup and single command execution | justfile (just 1.47.1 in nix profile); uv for Python venv (0.5.9 available); single `just all` command |
| DATA-08 | Pipeline validates all school records against Pydantic schema before writing | pydantic 2.12.5; validate before write pattern from ARCHITECTURE.md |
| DATA-09 | Pipeline collects ratings from multiple permitted sources with source attribution, scale, and review count | schulnoten.de is dead (404); schulinfo.de is for-sale; Tagesspiegel Abitur data is JS-rendered (needs web_search tool or browser); Google Maps forbidden by ToS |
| DATA-10 | Pipeline flags which schools accept students after 4th grade (grundständig) | Not in WFS data; must be determined by agent research per school |
| DATA-11 | Pipeline collects admission requirements per school where publicly available | Agent research via web_search tool; schulverzeichnis-berlin.de is unreachable (000); must rely on agent web research |
| DATA-12 | Pipeline collects open day (Tag der offenen Tür) dates from school websites | Agent research via web_search tool; JS-rendered school sites need web_fetch server tool |
</phase_requirements>

---

## Summary

The pipeline has three distinct source layers: a WFS seed (structured, machine-readable, confirmed working), an Eckdaten XLSX (student/teacher counts), and Claude agent enrichment for everything else. The Berlin Open Data WFS at `gdi.berlin.de/services/wfs/schulen` is the canonical seed. It returns 95 public + 11 private Gymnasien. All 15 property fields are available, including BSN, name, district, address, phone, email, website, and school year. Coordinates need `srsName=EPSG:4326` to get WGS84 lat/lng. The Eckdaten Excel (12KB) at `berlin.de/sen/bildung/service/daten/od-eckdaten-allg-2024.xlsx` provides student/teacher counts keyed by BSN.

Claude agent enrichment uses the `web_search_20260209` server tool built into the Anthropic API. This tool supports dynamic filtering on claude-sonnet-4-6 and replaces the need for external browser automation. It is the correct implementation of D-08. The Messages Batches API (available in anthropic SDK 0.86.0 via `client.beta.messages.batches`) can process up to 24 hours async, but for D-01/D-02 the synchronous asyncio + Semaphore pattern from ARCHITECTURE.md is simpler and provides the 5-10 min runtime target. The key infrastructure gap is that `just` and `uv` are available (nix profile and ~/.local/bin respectively) but neither `wasm32-unknown-unknown` target nor many Python pipeline packages are installed yet.

The biggest data risk is rating sources. Both schulnoten.de (dead, 404) and schulinfo.de (domain for sale) are gone. The rating source discovery agent (D-22) must run first before the enrichment phase. Tagesspiegel Abitur data (D-24) is JS-rendered and requires the web_search/web_fetch server tools, not static HTTP. The `grundständig` flag (DATA-10) is not in any structured source — it must come entirely from agent research.

**Primary recommendation:** Build pipeline in 4 sequential steps — seed (WFS + XLSX), structured enrichment, agent enrichment with web_search tool, then validate + write. Use `uv` for Python environment management and `just` for task runner. Use asyncio + Semaphore(8) for agent concurrency, not the Messages Batches API.

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Python | 3.13.1 | Pipeline language | Available via uv; required for anthropic SDK |
| anthropic | 0.86.0 | Claude API client — agent enrichment + web_search | Project constraint; web_search_20260209 server tool built in; no external browser needed |
| httpx | 0.28.1 | Async HTTP for WFS + XLSX fetching | Native async/await; used for concurrent structured scraping |
| beautifulsoup4 | 4.14.3 | HTML parsing fallback for school websites | Industry standard; fallback when web_search returns HTML |
| pydantic | 2.12.5 | Schema validation before YAML write | Generates JSON schema for Claude prompts; validates all records |
| ruamel.yaml | 0.19.1 | YAML read/write preserving formatting | Round-trip safe; comment-preserving; diff-friendly output |
| tenacity | 9.1.4 | Retry with exponential backoff (D-11) | Handles Claude API rate limits and transient errors |
| deepdiff | 8.6.2 | Nested field-level diff for changelog | Handles nested dicts/lists; produces changelog entries per field |
| openpyxl | 3.1.5 | Parse Eckdaten XLSX (student/teacher counts) | Required for reading berlin.de/sen/bildung Excel file |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| uv | 0.5.9 | Python venv + package management | Create isolated pipeline environment; faster than pip |
| just | 1.47.1 | Task runner (D-25) | `just seed`, `just scrape`, `just all` commands |
| lxml | latest | Faster XML/HTML parsing for WFS GeoJSON | Optional; httpx + json.loads is sufficient for WFS GeoJSON |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| asyncio + Semaphore | Messages Batches API | Batches API takes up to 24h; asyncio gives 5-10 min target runtime; batches only makes sense if budget > 24h wait |
| web_search server tool | playwright / pyppeteer | Server tool requires no local browser install; simpler; costs $10/1000 searches |
| deepdiff | pyyaml diff / manual | deepdiff handles nested structures automatically; no manual recursion needed |
| openpyxl | xlrd | openpyxl handles .xlsx (xlsx is current format); xlrd only handles .xls |

**Installation:**
```bash
# Create project virtual environment with uv
uv venv --python 3.13 .venv
source .venv/bin/activate

# Install pipeline dependencies
uv pip install anthropic==0.86.0 httpx==0.28.1 beautifulsoup4==4.14.3 pydantic==2.12.5 "ruamel.yaml==0.19.1" tenacity==9.1.4 deepdiff==8.6.2 openpyxl==3.1.5
```

**Version verification:** All versions confirmed via PyPI on 2026-03-26.

---

## Architecture Patterns

### Recommended Project Structure
```
pipeline/
├── run.py              # Orchestrator: asyncio main, argument parsing
├── pipeline.yaml       # Config (defaults, source URLs, semaphore limit)
├── seed.py             # WFS fetch + XLSX merge → schools_index.yaml
├── scrape.py           # Structured scraper (WFS already covers most fields)
├── agent.py            # Claude agent enrichment per batch
├── validate.py         # Pydantic schema + merger (structured wins over agent)
├── writer.py           # YAML write + deepdiff changelog
├── models.py           # Pydantic SchoolRecord model (single source of truth)
├── cache/              # Per-school JSON cache (D-09)
│   └── {school_id}.json
data/
├── schools_index.yaml  # Seed output: school_id, name, district, website, coords
├── schools/
│   └── {bsn}.yaml      # One file per school
├── CHANGELOG.md        # Appended per run
└── conflicts.md        # Regenerated per run
justfile               # Pipeline tasks
```

### Pattern 1: WFS Seed with EPSG:4326 Projection
**What:** Fetch all public Berlin Gymnasien from GDI Berlin WFS with lat/lng coordinates in WGS84
**When to use:** Step 1 of pipeline — produces schools_index.yaml
**Example:**
```python
# Source: verified via curl against gdi.berlin.de/services/wfs/schulen (2026-03-26)
import httpx, json

WFS_URL = "https://gdi.berlin.de/services/wfs/schulen"

async def fetch_gymnasien() -> list[dict]:
    params = {
        "service": "WFS",
        "version": "2.0.0",
        "request": "GetFeature",
        "typeNames": "schulen",
        "outputFormat": "application/json",
        "srsName": "EPSG:4326",
        "CQL_FILTER": "schulart='Gymnasium'",
    }
    async with httpx.AsyncClient() as client:
        resp = await client.get(WFS_URL, params=params, timeout=30)
        resp.raise_for_status()
        data = resp.json()
    schools = []
    for feature in data["features"]:
        props = feature["properties"]
        coords = feature["geometry"]["coordinates"]  # [lng, lat] in EPSG:4326
        schools.append({
            "school_id": props["bsn"],
            "name": props["schulname"].strip(),
            "district": props["bezirk"],
            "address": f"{props['strasse']} {props['hausnr'].strip()}, {props['plz']} Berlin",
            "phone": props.get("telefon"),
            "email": props.get("email"),
            "website": props.get("internet"),
            "traeger": props["traeger"],  # 'öffentlich' or 'privat'
            "coords": {"lat": coords[1], "lng": coords[0]},
        })
    return schools
```

### Pattern 2: Claude Agent with web_search Server Tool
**What:** Single API call that researches a batch of 5-10 schools and returns structured JSON
**When to use:** Agent enrichment step for all D-03/DATA-03 fields
**Example:**
```python
# Source: Anthropic API docs platform.claude.com (2026-03-26)
import anthropic, asyncio

client = anthropic.Anthropic()  # ANTHROPIC_API_KEY from env (D-27)

async def enrich_batch(batch: list[dict], semaphore: asyncio.Semaphore) -> list[dict]:
    """Research a batch of 5-10 schools and return enriched data."""
    async with semaphore:
        school_list = "\n".join(
            f"- {s['school_id']}: {s['name']}, website: {s.get('website', 'unknown')}"
            for s in batch
        )
        prompt = f"""Research the following Berlin Gymnasien and for each school find:
- grundständig flag (accepts students after 4th grade, not 6th)
- profile/specializations (MINT, bilingual, music, sports, altsprachlich, etc.)
- languages offered (name, from which grade)
- Ganztag status (full-day or half-day)
- next Tag der offenen Tür (open day) date
- admission requirements (Notendurchschnitt threshold, selection criteria)
- ratings from any school review sites
- image URLs, Instagram and Facebook links
- Abitur average if mentioned on Tagesspiegel or school site

Search in German. Return JSON array matching the school IDs provided.

Schools to research:
{school_list}

Return a JSON array with one object per school_id containing all found fields.
Set fields to null if not found after searching. Note confidence level (high/medium/low) per field."""

        # Use synchronous client in thread pool for asyncio compatibility
        loop = asyncio.get_event_loop()
        response = await loop.run_in_executor(
            None,
            lambda: client.messages.create(
                model="claude-sonnet-4-6",  # D-07
                max_tokens=4000,
                messages=[{"role": "user", "content": prompt}],
                tools=[{
                    "type": "web_search_20260209",  # D-08: server tool, no browser needed
                    "name": "web_search",
                    "max_uses": 10,
                    "user_location": {
                        "type": "approximate",
                        "city": "Berlin",
                        "country": "DE",
                        "timezone": "Europe/Berlin",
                    }
                }],
            )
        )
        # Extract JSON from response text blocks
        text = next(b.text for b in response.content if hasattr(b, "text"))
        # Parse JSON from text (agent returns JSON embedded in text)
        import re, json
        match = re.search(r'\[.*\]', text, re.DOTALL)
        return json.loads(match.group()) if match else []
```

### Pattern 3: asyncio Semaphore Concurrency
**What:** Bounded concurrency for agent batches (D-02: 8-10 parallel batches)
**When to use:** Orchestrator for the agent enrichment step
**Example:**
```python
# Source: ARCHITECTURE.md pattern (2026-03-26)
async def run_all_batches(schools: list[dict], batch_size: int = 8) -> list[dict]:
    sem = asyncio.Semaphore(8)  # D-02: 8-10 parallel batches
    batches = [schools[i:i+batch_size] for i in range(0, len(schools), batch_size)]
    results = await asyncio.gather(*[enrich_batch(b, sem) for b in batches])
    return [school for batch in results for school in batch]
```

### Pattern 4: Pydantic Validation + Structured Source Wins (D-17)
**What:** Merge structured (WFS/XLSX) data with agent data; structured source always wins
**When to use:** After both structured scrape and agent enrichment complete
**Example:**
```python
from pydantic import BaseModel, Field
from typing import Optional, Literal

class RatingEntry(BaseModel):
    source: str
    score: Optional[float]
    scale_min: float
    scale_max: float
    review_count: Optional[int]
    retrieved: str  # ISO date

class LanguageEntry(BaseModel):
    name: str
    from_grade: Optional[int]

class SchoolRecord(BaseModel):
    school_id: str
    name: str
    district: str
    address: Optional[str] = None
    coords: Optional[dict] = None
    website: Optional[str] = None
    phone: Optional[str] = None
    email: Optional[str] = None
    traeger: Optional[str] = None
    accepts_after_4th_grade: Optional[bool] = None  # DATA-10
    profile: list[str] = Field(default_factory=list)
    ganztag: Optional[bool] = None
    student_count: Optional[int] = None
    teacher_count: Optional[int] = None
    languages: list[LanguageEntry] = Field(default_factory=list)
    ratings: dict[str, RatingEntry] = Field(default_factory=dict)  # DATA-09
    open_day: Optional[str] = None  # DATA-12
    admission_requirements: Optional[dict] = None  # DATA-11
    image_urls: list[str] = Field(default_factory=list)  # D-14
    social_media: dict[str, str] = Field(default_factory=dict)  # D-14
    abitur_average: Optional[float] = None  # D-24
    completeness_score: Optional[float] = None  # D-15
    field_confidence: dict[str, Literal["high", "medium", "low"]] = Field(default_factory=dict)
    data_sources: list[str] = Field(default_factory=list)  # DATA-05
    last_updated: str  # ISO date
    unverified_fields: list[str] = Field(default_factory=list)  # D-19
```

### Pattern 5: Field-Level Changelog with deepdiff
**What:** Compare prior YAML to new record, write human-readable changelog
**When to use:** Writer step after each school YAML is computed
**Example:**
```python
from deepdiff import DeepDiff
from ruamel.yaml import YAML
from datetime import datetime, timezone
import pathlib

def compute_changelog_entry(school_id: str, name: str, prior: dict, new: dict) -> str:
    diff = DeepDiff(prior, new, ignore_order=True, view="text")
    if not diff:
        return ""
    lines = [f"### {school_id} — {name}"]
    for change_type, changes in diff.items():
        for path, change in (changes.items() if hasattr(changes, "items") else [(changes, "")]):
            if change_type == "values_changed":
                lines.append(f"- `{path}`: {change['old_value']!r} → {change['new_value']!r}")
            elif change_type == "dictionary_item_added":
                lines.append(f"- `{path}`: added {change!r}")
            elif change_type == "dictionary_item_removed":
                lines.append(f"- `{path}`: removed")
    return "\n".join(lines)
```

### Pattern 6: Tenacity Retry with Exponential Backoff (D-11)
**What:** 3 attempts with exponential backoff on Claude API errors
**When to use:** Wrap every `client.messages.create` call
**Example:**
```python
from tenacity import retry, stop_after_attempt, wait_exponential, retry_if_exception_type
import anthropic

@retry(
    stop=stop_after_attempt(3),
    wait=wait_exponential(multiplier=2, min=4, max=60),
    retry=retry_if_exception_type((anthropic.RateLimitError, anthropic.APIStatusError)),
)
def create_message_with_retry(client, **kwargs):
    return client.messages.create(**kwargs)
```

### Anti-Patterns to Avoid
- **Writing YAML during concurrent scraping:** Multiple writers corrupt files. Return data in-memory; single Writer serializes at end (ARCHITECTURE.md Anti-Pattern 3).
- **Storing WFS coordinates without EPSG:4326:** The WFS returns EPSG:25833 (UTM Zone 33N) by default. Always include `srsName=EPSG:4326` in the request.
- **Assuming schulnoten.de or schulinfo.de work:** Both sites are dead as of 2026-03-26. Rating source discovery must be done by the agent.
- **Using `import ruamel.yaml; yaml.safe_dump`:** ruamel.yaml uses `YAML()` object, not the pyyaml-compatible API. Use `YAML(typ='rt')` for round-trip and `YAML(typ='safe')` for new files.
- **Calling `client.messages.create` directly in asyncio coroutines:** The anthropic Python SDK is synchronous. Use `loop.run_in_executor(None, lambda: client.messages.create(...))` to avoid blocking the event loop.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Nested dict diff for changelog | Custom recursive differ | deepdiff 8.6.2 | Handles lists, sets, nested dicts; produces structured change report |
| Web browser automation | playwright/pyppeteer | web_search_20260209 server tool | No local browser install; server-side execution; $10/1000 searches |
| JSON schema from Pydantic model | Manual JSON schema | `SchoolRecord.model_json_schema()` | Pydantic generates valid JSON schema for use in Claude prompts |
| YAML round-trip preserving comments | Custom YAML serializer | ruamel.yaml | Handles YAML comments, block scalars, ordering; PyYAML destroys comments |
| Exponential backoff retry logic | time.sleep + try/except loop | tenacity | Handles jitter, max attempts, exception filtering correctly |
| Coordinate conversion | pyproj / manual math | Request srsName=EPSG:4326 in WFS call | WFS server handles projection; no library needed |
| Excel parsing for Eckdaten | CSV export + manual | openpyxl | File is native .xlsx format; openpyxl reads it directly |

**Key insight:** The Anthropic API's web_search server tool is the correct answer to D-08. It eliminates the need for local browser automation (playwright, selenium) and handles JS-rendered sites that would otherwise require a headless browser.

---

## Common Pitfalls

### Pitfall 1: WFS Coordinates Are EPSG:25833 by Default, Not WGS84
**What goes wrong:** WFS returns coordinates like `[388591.4, 5820578.3]` (meters, UTM Zone 33N). Trying to use these as lat/lng produces nonsense location data.
**Why it happens:** GDI Berlin serves data in Berlin's native projection.
**How to avoid:** Always include `srsName=EPSG:4326` in the WFS GetFeature request. Verified: this works and returns `[13.399, 52.524]` style WGS84 coordinates.
**Warning signs:** Coordinates with values > 1000 are in a projected CRS, not WGS84.

### Pitfall 2: Rating Sites Are Dead — Don't Assume Any Exist
**What goes wrong:** Planning assumes schulnoten.de or schulinfo.de are valid rating sources. Both are defunct as of 2026-03-26 (schulnoten.de returns 404; schulinfo.de is "domain for sale"). Building scrapers for them wastes time.
**Why it happens:** Research documents from early 2026 referenced them; they died between research and implementation.
**How to avoid:** D-22 mandates a rating source discovery agent run before the enrichment phase. The agent must find current working sources. Accept that v1 may ship without third-party ratings. Use Abitur averages from Tagesspiegel as the primary quality proxy (D-24).
**Warning signs:** 404 or "domain for sale" page content from known rating site URLs.

### Pitfall 3: Tagesspiegel Abitur Data Is Fully JS-Rendered
**What goes wrong:** Static HTTP fetch to the Tagesspiegel interactive map returns the page shell without any school data embedded in the HTML. The data loads via JavaScript. `curl` or `httpx` gets zero data.
**Why it happens:** Interactive data journalism tools render data client-side.
**How to avoid:** Use the `web_search_20260209` server tool to research Abitur averages per school. The tool internally fetches and processes JS-rendered content. Alternatively, search for the underlying data API (Tagesspiegel sometimes exposes data JSON files).
**Warning signs:** HTTP 200 but zero school records in parsed HTML; "content length: 107169" with no structured data.

### Pitfall 4: anthropic SDK is Synchronous — Blocks Event Loop in asyncio
**What goes wrong:** Calling `client.messages.create(...)` directly inside an `async def` function blocks the event loop, serializing all agent calls despite asyncio.gather.
**Why it happens:** anthropic Python SDK uses synchronous HTTP (httpx sync) by default.
**How to avoid:** Either use `AsyncAnthropic` client (available in 0.86.0), or wrap synchronous calls in `loop.run_in_executor(None, lambda: ...)`. The `AsyncAnthropic` client is cleaner and preferred.
**Warning signs:** asyncio.gather completes sequentially (same wall time as sequential execution).

### Pitfall 5: WFS Has 106 Gymnasien — Include Private Schools Decision
**What goes wrong:** The WFS returns 95 public and 11 private Gymnasien (total 106). The project scope says "~90 Berlin Gymnasien" without specifying public-only. Including private schools adds 11 more but they may have incomplete public data.
**Why it happens:** Ambiguity in "Berlin Gymnasien" — the full set is 106.
**How to avoid:** Include all 106 by default (the 11 private ones still useful for parents). Filter by `traeger='öffentlich'` only if explicitly required. Document in pipeline.yaml whether private schools are included.
**Warning signs:** School count is 90 or 95 exactly — check if private schools were accidentally excluded.

### Pitfall 6: `grundständig` Flag Cannot Be Derived from Any Structured Source
**What goes wrong:** Assuming the WFS or Eckdaten XLSX contains a `grundständig` field. Neither source has this field. Building a lookup table without agent research produces missing data for DATA-10.
**Why it happens:** It's a critical data point but not in official machine-readable datasets.
**How to avoid:** Each agent batch must specifically search for whether each school accepts students after 4th grade. The Berlin Senate list of grundständige Gymnasien is a scraped HTML source (berlin.de) — agent can find it via web_search. Cross-reference D-17: if found in both structured and agent, structured wins (but there is no structured source here, so agent result is authoritative).
**Warning signs:** `accepts_after_4th_grade` field is null for all schools after pipeline run.

### Pitfall 7: schulverzeichnis.berlin.de Is Unreachable
**What goes wrong:** ARCHITECTURE.md and STACK.md reference `schulverzeichnis.berlin.de` as a structured HTML source for admission criteria. This domain returns connection refused (000 error) as of 2026-03-26.
**Why it happens:** The site may be down, renamed, or behind a VPN requirement.
**How to avoid:** Do not plan a structured scraper step for schulverzeichnis.berlin.de. Rely on agent web_search for admission criteria (DATA-11). The WFS already covers address/phone/email/website. The Eckdaten XLSX covers student/teacher counts.
**Warning signs:** `httpx.ConnectError` or timeout when fetching schulverzeichnis.berlin.de.

### Pitfall 8: Conflict Between YAML Schema and Manual Edits on Re-Runs
**What goes wrong:** If the YAML files are manually edited (e.g., to correct a wrong grundständig flag), re-running the pipeline overwrites the manual corrections with agent data.
**Why it happens:** Pipeline writes from fresh agent data without respecting manual overrides.
**How to avoid:** Add an `overrides/` directory or a `_manual_overrides` section in each YAML. Writer step reads overrides and applies them last, after merge + validation. This is "Claude's Discretion" — pick a simple strategy (e.g., `_pinned_fields: [field1, field2]` in YAML that the writer never overwrites).
**Warning signs:** Manual corrections disappear after pipeline re-run.

---

## Code Examples

### Invoking web_search_20260209 Server Tool
```python
# Source: platform.claude.com/docs/en/docs/build-with-claude/tool-use/web-search-tool (2026-03-26)
import anthropic

client = anthropic.AsyncAnthropic()  # Use async client to avoid blocking event loop

response = await client.messages.create(
    model="claude-sonnet-4-6",
    max_tokens=4000,
    messages=[{"role": "user", "content": "Find information about Gymnasium X in Berlin."}],
    tools=[{
        "type": "web_search_20260209",  # Latest version with dynamic filtering
        "name": "web_search",
        "max_uses": 10,
        "user_location": {
            "type": "approximate",
            "city": "Berlin",
            "country": "DE",
            "timezone": "Europe/Berlin",
        }
    }],
)
# Response contains text blocks with citations and server_tool_use blocks
```

### Reading Eckdaten XLSX with openpyxl
```python
# Source: openpyxl docs; file URL verified 2026-03-26
import httpx, io, openpyxl

async def fetch_eckdaten() -> dict[str, dict]:
    """Returns dict keyed by BSN with student_count, teacher_count."""
    url = "https://www.berlin.de/sen/bildung/service/daten/od-eckdaten-allg-2024.xlsx"
    async with httpx.AsyncClient() as client:
        resp = await client.get(url, timeout=30)
        resp.raise_for_status()
    wb = openpyxl.load_workbook(io.BytesIO(resp.content))
    ws = wb.active
    headers = [cell.value for cell in next(ws.iter_rows(min_row=1, max_row=1))]
    result = {}
    for row in ws.iter_rows(min_row=2, values_only=True):
        row_dict = dict(zip(headers, row))
        bsn = row_dict.get("BSN") or row_dict.get("Schulnummer")
        if bsn:
            result[str(bsn)] = row_dict
    return result
```

### Writing YAML with ruamel.yaml
```python
# Source: ruamel.yaml docs; round-trip pattern
from ruamel.yaml import YAML
import pathlib

yaml = YAML()
yaml.default_flow_style = False
yaml.width = 120

def write_school_yaml(record: dict, path: pathlib.Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        yaml.dump(record, f)

def read_school_yaml(path: pathlib.Path) -> dict:
    if not path.exists():
        return {}
    with path.open("r", encoding="utf-8") as f:
        return yaml.load(f) or {}
```

### justfile Structure (D-25)
```just
# Source: CONTEXT.md D-25; just 1.47.1 syntax
# Requires: ANTHROPIC_API_KEY env var (D-27)

default: all

seed:
    uv run python pipeline/run.py --step seed

scrape:
    uv run python pipeline/run.py --step scrape

enrich:
    uv run python pipeline/run.py --step enrich

validate:
    uv run python pipeline/run.py --step validate

write:
    uv run python pipeline/run.py --step write

all:
    uv run python pipeline/run.py --step all

force:
    uv run python pipeline/run.py --step all --force
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| playwright/selenium for JS pages | web_search_20260209 server tool | 2026-02 (tool version update) | No local browser install; simpler; $10/1000 searches |
| anthropic SDK < 0.40 (no tool use) | anthropic 0.86.0 with server tools | 2026 | web_search, web_fetch, code_execution built in |
| serde_yaml in Rust | serde-saphyr | 2023 (serde_yaml deprecated) | Only relevant for Phase 2 SPA; not pipeline |
| Manual WFS projection | srsName=EPSG:4326 parameter | OGC WFS 1.1+ | Server reprojects; no client-side conversion library needed |

**Deprecated/outdated:**
- schulnoten.de: Dead (404 as of 2026-03-26). Do not reference.
- schulinfo.de: Domain for sale as of 2026-03-26. Do not reference.
- bewertet.de/schulen: Redirecting to ProvenExpert (platform closing). Do not reference.
- schulverzeichnis.berlin.de: Unreachable (connection refused). Remove from structured scraper plan.

---

## Open Questions

1. **Rating source discovery (D-22)**
   - What we know: All pre-identified third-party rating sites are dead or inaccessible
   - What's unclear: Whether any current German school rating platform covers Berlin Gymnasien
   - Recommendation: Agent discovery run (D-22) is mandatory before enrichment phase. Accept that v1 may have ratings field empty for most schools, relying on Abitur averages (D-24) and proxy signals (D-23).

2. **Tagesspiegel Abitur data extraction strategy (D-24)**
   - What we know: Page is fully JS-rendered; static HTTP gets no data; page confirmed reachable (200)
   - What's unclear: Whether the web_search server tool can extract structured data from this page, or whether we need to find an underlying data API
   - Recommendation: Let the agent use web_search with specific queries like "Tagesspiegel Abitur 2025 {school_name} Berlin Durchschnitt". The tool may find the data in search results even if the interactive map itself is JS-only. Budget 2-3 web_search uses per school for Tagesspiegel data.

3. **Private schools inclusion scope**
   - What we know: WFS has 95 public + 11 private Gymnasien (106 total)
   - What's unclear: Whether the 11 private schools are in scope ("~90 Berlin Gymnasien" is ambiguous)
   - Recommendation: Include all 106 in the pipeline. The product's audience includes parents choosing private as well as public. Document the traeger field; let the frontend filter if needed.

4. **Manual override strategy for re-runs (Claude's Discretion)**
   - What we know: Re-runs should not destroy manually corrected data
   - What's unclear: How granular overrides should be (per-field vs per-school)
   - Recommendation: Implement `_pinned_fields` list in each YAML file. Writer step skips any field listed in `_pinned_fields` when merging agent data. Simple and explicit.

5. **Web search pricing impact on $20 budget (D-13)**
   - What we know: web_search costs $10/1000 searches; max_uses=10 per school; 106 schools → up to 1060 searches → $10.60 for web_search alone
   - What's unclear: Claude token costs for the agent calls on top of search costs
   - Recommendation: Set max_uses=5 per school (not 10) to stay within budget. 106 schools × 5 searches × $0.01 = $5.30 for web_search. Claude token costs for Sonnet 4.6 at ~4000 tokens/school × 13 batches ≈ ~$3-5 in tokens. Total should stay under $15.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Python 3.12+ | Pipeline runtime | Yes (via uv) | 3.13.1 at ~/.local/share/uv/python | `uv python install 3.13` |
| uv | Python env management | Yes | 0.5.9 at ~/.local/bin/uv | pip + venv (slower) |
| just | Task runner (D-25) | Yes | 1.47.1 at ~/.nix-profile/bin/just | make, or run python directly |
| anthropic SDK | Agent enrichment | Not installed | 0.86.0 on PyPI | `uv pip install anthropic` |
| httpx | WFS + XLSX fetch | Not installed | 0.28.1 on PyPI | `uv pip install httpx` |
| pydantic | Schema validation | Not installed | 2.12.5 on PyPI | `uv pip install pydantic` |
| ruamel.yaml | YAML write/read | Not installed | 0.19.1 on PyPI | `uv pip install ruamel.yaml` |
| openpyxl | Eckdaten XLSX | Not installed | 3.1.5 on PyPI | `uv pip install openpyxl` |
| deepdiff | Changelog | Not installed | 8.6.2 on PyPI | `uv pip install deepdiff` |
| tenacity | Retry logic | Not installed | 9.1.4 on PyPI | `uv pip install tenacity` |
| cargo/rustc | Not needed for Phase 1 | Yes | 1.94.0 (nix profile) | n/a |
| trunk | Not needed for Phase 1 | Yes | 0.21.14 (cargo bin) | n/a |
| ANTHROPIC_API_KEY | Agent enrichment | Unknown | — | Must be set before pipeline run |
| GDI Berlin WFS | Seed step | Yes | `gdi.berlin.de/services/wfs/schulen` confirmed 200 | None — blocking if down |
| Eckdaten XLSX | Student/teacher counts | Yes | `berlin.de/sen/bildung/service/daten/od-eckdaten-allg-2024.xlsx` confirmed 200 | Omit student/teacher count fields |

**Missing dependencies with no fallback:**
- ANTHROPIC_API_KEY: Must be set in environment before running enrichment step. Pipeline should fail fast with clear error if missing.
- GDI Berlin WFS: No alternative source provides coordinates + structured fields for all 106 Gymnasien in one call.

**Missing dependencies with fallback:**
- All Python packages: installable via `uv pip install` in the Wave 0 setup task.
- wasm32-unknown-unknown Rust target: not needed for Phase 1; needed for Phase 2.

---

## Project Constraints (from CLAUDE.md)

- Tech stack (pipeline): Python + anthropic SDK (confirmed by STACK.md; locked)
- Data format: YAML files in `data/schools/` (locked)
- justfile for pipeline tasks (D-25, locked)
- Secrets via environment variables only (D-27, locked)
- No Google Maps scraping (REQUIREMENTS.md out-of-scope; legal constraint)
- No user accounts, no backend (out-of-scope)
- GSD workflow: use `/gsd:execute-phase` for implementation; do not make direct repo edits outside GSD workflow

---

## Sources

### Primary (HIGH confidence)
- GDI Berlin WFS endpoint: `https://gdi.berlin.de/services/wfs/schulen` — tested with GetFeature, confirmed 95+11 Gymnasien, EPSG:4326 output verified
- Berlin Eckdaten XLSX: `https://www.berlin.de/sen/bildung/service/daten/od-eckdaten-allg-2024.xlsx` — confirmed 200, valid XLSX format
- Anthropic API web_search tool docs: `https://platform.claude.com/docs/en/docs/build-with-claude/tool-use/web-search-tool` — web_search_20260209 with dynamic filtering confirmed for claude-sonnet-4-6
- PyPI version checks: anthropic 0.86.0, httpx 0.28.1, pydantic 2.12.5, ruamel.yaml 0.19.1, tenacity 9.1.4, beautifulsoup4 4.14.3, deepdiff 8.6.2, openpyxl 3.1.5 — all verified 2026-03-26
- Environment: uv 0.5.9, just 1.47.1, rustc 1.94.0, trunk 0.21.14 — all verified via command execution

### Secondary (MEDIUM confidence)
- ARCHITECTURE.md, STACK.md, PITFALLS.md, FEATURES.md — project research documents from 2026-03-26; core patterns verified against live APIs
- anthropic SDK source code at github.com/anthropics/anthropic-sdk-python — ServerToolUseBlock with web_search, web_fetch, code_execution confirmed in types

### Tertiary (LOW confidence)
- Tagesspiegel data extraction strategy — page structure confirmed JS-rendered; specific web_search query approach for Abitur data is a hypothesis based on tool documentation, not tested
- Budget estimate ($15 total) — based on published pricing ($10/1000 searches) and token cost estimates; actual cost depends on response length

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all versions verified via PyPI on 2026-03-26; environment tools confirmed present
- Architecture: HIGH — WFS schema verified with live query; asyncio + Semaphore pattern from ARCHITECTURE.md; web_search tool confirmed in API docs
- Pitfalls: HIGH for WFS projection, dead rating sites, unreachable schulverzeichnis; MEDIUM for manual override strategy (Claude's Discretion)
- Rating sources: LOW — all pre-identified sources are dead; discovery depends on agent run (D-22)

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable APIs, but rating source landscape could change weekly)
