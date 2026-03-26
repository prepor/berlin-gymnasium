"""
pipeline/agent.py — Agent enrichment step using Claude + web_search_20260209.

Uses AsyncAnthropic client (avoids blocking event loop — Pitfall 4).
Batches 5-10 schools per API call (D-01).
8 parallel batches via asyncio.Semaphore (D-02).
Caches results per school_id in pipeline/cache/ (D-09).
Retries on rate limit/API errors with exponential backoff (D-11).
"""
from __future__ import annotations

import asyncio
import json
import logging
import os
import re
from datetime import date
from pathlib import Path
from typing import Any

import anthropic
from tenacity import retry, retry_if_exception_type, stop_after_attempt, wait_exponential

logger = logging.getLogger(__name__)

MODEL = "claude-sonnet-4-6"  # D-07


def get_client() -> anthropic.AsyncAnthropic:
    """Return AsyncAnthropic client. Fails fast if ANTHROPIC_API_KEY not set (D-27)."""
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        raise EnvironmentError(
            "ANTHROPIC_API_KEY environment variable is not set.\n"
            "Set it before running the enrich step: export ANTHROPIC_API_KEY=sk-ant-..."
        )
    return anthropic.AsyncAnthropic(api_key=api_key)


def cache_path(school_id: str, cache_dir: str) -> Path:
    return Path(cache_dir) / f"{school_id}.json"


def read_cache(school_id: str, cache_dir: str) -> dict | None:
    p = cache_path(school_id, cache_dir)
    if p.exists():
        try:
            return json.loads(p.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError):
            return None
    return None


def write_cache(school_id: str, data: dict, cache_dir: str) -> None:
    p = cache_path(school_id, cache_dir)
    p.parent.mkdir(parents=True, exist_ok=True)
    entry = {"school_id": school_id, "enriched_at": date.today().isoformat(), "data": data}
    p.write_text(json.dumps(entry, ensure_ascii=False, indent=2), encoding="utf-8")


def build_prompt(batch: list[dict]) -> str:
    """Build the agent prompt for a batch of schools. (D-12: English prompt, German search)"""
    school_list = "\n".join(
        f"- school_id={s['school_id']}: {s['name']}, district={s.get('district')}, "
        f"website={s.get('website') or 'unknown'}"
        for s in batch
    )
    return f"""You are researching Berlin Gymnasien (secondary schools) for a parent comparison website.
For each school below, search the web (search in German) and find the following information.
Report all findings in English. Set fields to null if not found after searching.

Schools to research:
{school_list}

For EACH school, find:
1. accepts_after_4th_grade (bool): Does this school accept students after 4th grade (grundständig)?
   Search: "{{school_name}} grundständig" or "{{school_name}} Aufnahme Klasse 5"
   Source: Official berlin.de list of grundständige Gymnasien is most reliable.

2. profile (list of strings): School specializations. Values from: ["MINT", "music", "sports",
   "bilingual_english", "bilingual_french", "bilingual_other", "altsprachlich", "kunst", "IB", "other"]
   Search: school website, official description.

3. languages (list): Foreign languages offered. Format: [{{"name": "Englisch", "from_grade": 5}}]
   Search: school website curriculum/Fächer section.

4. ganztag (bool): Is this a full-day (Ganztagsschule) or half-day school?

5. open_day (string): Next Tag der offenen Tür date in ISO format "YYYY-MM-DD".
   Search: "{{school_name}} Tag der offenen Tür 2026" on school website.
   Set null if past or not found.

6. admission_requirements: Object with:
   - notendurchschnitt (float or null): GPA threshold for admission
   - oversubscribed (bool or null): typically oversubscribed?
   - selection_criteria (string or null): description of selection process
   - probeunterricht (bool or null): trial lesson required?
   - entrance_test (bool or null): entrance exam required?
   - notes (string or null): any other admission info

7. ratings: Dict keyed by source name. DO NOT use Google Maps (Terms of Service violation).
   Look for German school rating sites (NOT schulnoten.de or schulinfo.de — both defunct).
   Format per source: {{"source": "site_name", "score": 4.2, "scale_min": 1.0, "scale_max": 5.0,
   "review_count": 45, "retrieved": "2026-03-26"}}

8. abitur_average (float or null): Most recent Abitur average grade for this school.
   Search: "{{school_name}} Abitur Durchschnitt Tagesspiegel" or school website.

9. image_urls (list of strings): Up to 3 URLs to school photos from the school's own website.

10. social_media (dict): {{"instagram": "url", "facebook": "url"}} — only if official school accounts.

11. unverified_fields (list of strings): Field names where data came only from agent research,
    not cross-verified with an official source (for transparency per D-19).

12. field_confidence (dict): For each field you found data for, rate confidence:
    "high" = official source (school website, berlin.de), "medium" = secondary source,
    "low" = single mention or inferred.

Return a JSON array with one object per school_id. Example structure:
[
  {{
    "school_id": "01k01",
    "accepts_after_4th_grade": true,
    "profile": ["MINT", "bilingual_english"],
    "languages": [{{"name": "Englisch", "from_grade": 5}}, {{"name": "Französisch", "from_grade": 7}}],
    "ganztag": false,
    "open_day": "2026-11-15",
    "admission_requirements": {{
      "notendurchschnitt": 2.3,
      "oversubscribed": true,
      "selection_criteria": "Lottery among qualified applicants",
      "probeunterricht": false,
      "entrance_test": false,
      "notes": null
    }},
    "ratings": {{}},
    "abitur_average": 2.1,
    "image_urls": ["https://school.de/images/school.jpg"],
    "social_media": {{"instagram": "https://instagram.com/school"}},
    "unverified_fields": ["open_day", "abitur_average"],
    "field_confidence": {{"accepts_after_4th_grade": "high", "profile": "medium"}}
  }}
]

Return ONLY the JSON array. No markdown fences, no explanation text outside the array."""


async def _call_api(
    client: anthropic.AsyncAnthropic,
    batch: list[dict],
    max_uses: int,
) -> list[dict]:
    """
    Make one API call for a batch. Retried by enrich_batch wrapper.
    Returns parsed list of enriched dicts.
    """
    response = await client.messages.create(
        model=MODEL,
        max_tokens=8000,
        messages=[{"role": "user", "content": build_prompt(batch)}],
        tools=[{
            "type": "web_search_20250305",
            "name": "web_search",
            "max_uses": max_uses,
            "user_location": {
                "type": "approximate",
                "city": "Berlin",
                "country": "DE",
                "timezone": "Europe/Berlin",
            },
        }],
    )

    # Extract text from response (may contain multiple content blocks)
    text = ""
    for block in response.content:
        if hasattr(block, "text"):
            text += block.text

    # Parse JSON array from response text
    match = re.search(r'\[[\s\S]*\]', text)
    if not match:
        logger.warning("Agent returned no JSON array. Response text (truncated): %s", text[:500])
        return []
    try:
        return json.loads(match.group())
    except json.JSONDecodeError as e:
        logger.warning("Failed to parse agent JSON: %s. Text: %s", e, text[:500])
        return []


async def enrich_batch(
    batch: list[dict],
    semaphore: asyncio.Semaphore,
    client: anthropic.AsyncAnthropic,
    config: dict,
    force: bool = False,
) -> list[dict]:
    """
    Enrich a batch of schools. Returns list of enriched dicts (one per school in batch).
    Skips schools with valid cache unless force=True (D-09).
    """
    cache_dir = config.get("cache_dir", "pipeline/cache")
    max_uses = config.get("max_web_searches_per_school", 5)

    # Split batch into cached and uncached
    uncached = []
    results: list[dict] = []
    for school in batch:
        cached = read_cache(school["school_id"], cache_dir) if not force else None
        if cached:
            logger.info("Cache hit: %s", school["school_id"])
            results.append({"school_id": school["school_id"], **cached["data"]})
        else:
            uncached.append(school)

    if not uncached:
        return results

    async with semaphore:
        logger.info("Enriching batch of %d schools: %s", len(uncached), [s["school_id"] for s in uncached])

        # Retry wrapper (D-11)
        @retry(
            stop=stop_after_attempt(5),
            wait=wait_exponential(multiplier=2, min=15, max=120),
            retry=retry_if_exception_type((anthropic.RateLimitError, anthropic.APIStatusError)),
        )
        async def call_with_retry() -> list[dict]:
            return await _call_api(client, uncached, max_uses)

        try:
            enriched = await call_with_retry()
        except Exception as e:
            logger.error(
                "Agent enrichment failed for batch %s after 3 retries: %s. Flagging schools.",
                [s["school_id"] for s in uncached], e,
            )
            # Return stubs with null enrichment so pipeline can continue (D-11)
            enriched = [{"school_id": s["school_id"]} for s in uncached]

    # Write cache for successfully enriched schools
    enriched_by_id = {r.get("school_id"): r for r in enriched}
    for school in uncached:
        sid = school["school_id"]
        data = enriched_by_id.get(sid, {})
        if data:
            write_cache(sid, data, cache_dir)

    results.extend(enriched)
    return results


async def run_enrich(schools: list[dict], config: dict, force: bool = False) -> list[dict]:
    """
    Run enrichment for all schools. Returns list of enriched dicts (by school_id).
    Called by pipeline/run.py orchestrator.
    """
    client = get_client()
    batch_size = config.get("batch_size", 8)
    semaphore_limit = config.get("semaphore_limit", 8)

    sem = asyncio.Semaphore(semaphore_limit)
    batches = [schools[i:i + batch_size] for i in range(0, len(schools), batch_size)]
    logger.info(
        "Enriching %d schools in %d batches (semaphore=%d, force=%s)",
        len(schools), len(batches), semaphore_limit, force,
    )

    batch_results = await asyncio.gather(
        *[enrich_batch(b, sem, client, config, force) for b in batches],
        return_exceptions=True,
    )

    results = []
    for r in batch_results:
        if isinstance(r, Exception):
            logger.error("Batch failed: %s", r)
        else:
            results.extend(r)

    logger.info("Enrichment complete: %d schools enriched", len(results))
    print(f"Enrich complete: {len(results)}/{len(schools)} schools enriched")
    return results


if __name__ == "__main__":
    """Standalone test: enrich a single test school."""
    import sys
    from pathlib import Path
    from ruamel.yaml import YAML

    logging.basicConfig(level=logging.INFO, format="%(levelname)s %(message)s")
    yaml = YAML()
    config_path = Path("pipeline/pipeline.yaml")
    config = yaml.load(config_path) if config_path.exists() else {}

    # Use first school from index for testing
    index_path = Path(config.get("index_file", "data/schools_index.yaml"))
    if not index_path.exists():
        print("Run 'just seed' first to create schools_index.yaml")
        sys.exit(1)

    schools = yaml.load(index_path)
    # Pass --force to bypass cache, --all to process all schools
    force = "--force" in sys.argv
    results = asyncio.run(run_enrich(schools, config, force=force))
    print(json.dumps(results, indent=2, ensure_ascii=False))
