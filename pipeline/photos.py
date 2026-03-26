"""
pipeline/photos.py — Dedicated photo enrichment step using Claude + web_search.

The main enrichment agent has 12 fields to research with limited web searches,
so photos rarely get found. This step focuses exclusively on finding good
school building/campus photos for each school.

Strategies:
1. Visit the school's own website for gallery/press/about photos
2. Search Wikimedia Commons for the school building
3. Web search for "[school name] Berlin Gebäude/Schulgebäude"
4. Validate URLs are direct image links
"""
from __future__ import annotations

import asyncio
import json
import logging
import re
from datetime import date
from pathlib import Path
from typing import Any

import anthropic
from tenacity import retry, retry_if_exception_type, stop_after_attempt, wait_exponential

from pipeline.agent import get_client, read_cache, write_cache

logger = logging.getLogger(__name__)

MODEL = "claude-sonnet-4-6"
PHOTO_CACHE_PREFIX = "photos_"


def photo_cache_path(school_id: str, cache_dir: str) -> Path:
    return Path(cache_dir) / f"{PHOTO_CACHE_PREFIX}{school_id}.json"


def read_photo_cache(school_id: str, cache_dir: str) -> list[str] | None:
    p = photo_cache_path(school_id, cache_dir)
    if p.exists():
        try:
            data = json.loads(p.read_text(encoding="utf-8"))
            return data.get("image_urls")
        except (json.JSONDecodeError, OSError):
            return None
    return None


def write_photo_cache(school_id: str, image_urls: list[str], cache_dir: str) -> None:
    p = photo_cache_path(school_id, cache_dir)
    p.parent.mkdir(parents=True, exist_ok=True)
    entry = {
        "school_id": school_id,
        "enriched_at": date.today().isoformat(),
        "image_urls": image_urls,
    }
    p.write_text(json.dumps(entry, ensure_ascii=False, indent=2), encoding="utf-8")


def build_photo_prompt(batch: list[dict]) -> str:
    """Build a focused prompt for finding school photos."""
    school_list = "\n".join(
        f"- school_id={s['school_id']}: {s['name']}, "
        f"website={s.get('website') or 'unknown'}, "
        f"address={s.get('address') or 'unknown'}"
        for s in batch
    )
    return f"""You are finding photos of Berlin Gymnasium school buildings for a parent comparison website.
For each school below, find 1-3 good photo URLs showing the school building, campus, or entrance.

Schools to find photos for:
{school_list}

## Search Strategy (follow this order)

1. **School website first**: Visit the school's website. Look for:
   - Homepage hero/banner images showing the building
   - "Über uns" / "About" / "Unsere Schule" pages with building photos
   - Gallery / "Galerie" / "Bilder" sections
   - Press / "Presse" section with photos
   Extract the direct image URL (ending in .jpg, .jpeg, .png, .webp).

2. **Wikimedia Commons**: Search for the school name on Wikimedia Commons.
   Many Berlin school buildings have photos uploaded by Wikipedia contributors.
   Use URLs from upload.wikimedia.org (these are stable, freely licensed).

3. **Web search**: Search for "{{school_name}} Berlin Schulgebäude" or "{{school_name}} Berlin Gymnasium Gebäude".
   Look for images from news articles, Berlin.de pages, or educational portals.

## Requirements for URLs

- Must be direct image URLs (the URL itself loads the image, not an HTML page)
- Prefer URLs ending in .jpg, .jpeg, .png, or .webp
- Prefer exterior/building shots over classroom or event photos
- Do NOT use Google Maps Street View URLs
- Do NOT use URLs from social media (Instagram, Facebook) — they expire
- Do NOT use thumbnail URLs — prefer full-size images
- Wikimedia Commons URLs are excellent (stable, high quality, freely licensed)

## Output Format

Return a JSON array with one object per school:
[
  {{
    "school_id": "01k01",
    "image_urls": ["https://example.com/school-building.jpg"],
    "image_sources": ["school_website"]
  }}
]

image_sources values: "school_website", "wikimedia_commons", "news_article", "berlin_de", "other"

If you genuinely cannot find any photo for a school after searching, return an empty list for that school.
But try hard — most schools have at least one photo of their building somewhere on the internet.

Return ONLY the JSON array. No markdown fences, no explanation."""


async def _call_photo_api(
    client: anthropic.AsyncAnthropic,
    batch: list[dict],
    max_uses: int,
) -> list[dict]:
    """Make one API call for a batch of schools to find photos."""
    response = await client.messages.create(
        model=MODEL,
        max_tokens=4000,
        messages=[{"role": "user", "content": build_photo_prompt(batch)}],
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

    text = ""
    for block in response.content:
        if hasattr(block, "text"):
            text += block.text

    match = re.search(r'\[[\s\S]*\]', text)
    if not match:
        logger.warning("Photo agent returned no JSON array. Text: %s", text[:500])
        return []
    try:
        return json.loads(match.group())
    except json.JSONDecodeError as e:
        logger.warning("Failed to parse photo JSON: %s. Text: %s", e, text[:500])
        return []


async def enrich_photos_batch(
    batch: list[dict],
    semaphore: asyncio.Semaphore,
    client: anthropic.AsyncAnthropic,
    config: dict,
    force: bool = False,
) -> list[dict]:
    """Find photos for a batch of schools."""
    cache_dir = config.get("cache_dir", "pipeline/cache")
    # More searches per school since we're only looking for photos
    max_uses = config.get("max_photo_searches_per_school", 3) * len(batch)

    uncached = []
    results: list[dict] = []
    for school in batch:
        if not force:
            cached = read_photo_cache(school["school_id"], cache_dir)
            if cached is not None:
                logger.info("Photo cache hit: %s (%d photos)", school["school_id"], len(cached))
                results.append({"school_id": school["school_id"], "image_urls": cached})
                continue
        uncached.append(school)

    if not uncached:
        return results

    async with semaphore:
        logger.info("Finding photos for %d schools: %s", len(uncached), [s["school_id"] for s in uncached])

        @retry(
            stop=stop_after_attempt(5),
            wait=wait_exponential(multiplier=2, min=15, max=120),
            retry=retry_if_exception_type((anthropic.RateLimitError, anthropic.APIStatusError)),
        )
        async def call_with_retry() -> list[dict]:
            return await _call_photo_api(client, uncached, max_uses)

        try:
            enriched = await call_with_retry()
        except Exception as e:
            logger.error("Photo enrichment failed for batch %s: %s", [s["school_id"] for s in uncached], e)
            enriched = [{"school_id": s["school_id"], "image_urls": []} for s in uncached]

    # Cache results
    enriched_by_id = {r.get("school_id"): r for r in enriched}
    for school in uncached:
        sid = school["school_id"]
        data = enriched_by_id.get(sid, {})
        urls = data.get("image_urls", [])
        write_photo_cache(sid, urls, cache_dir)
        results.append({"school_id": sid, "image_urls": urls})

    return results


def load_schools_needing_photos(config: dict) -> list[dict]:
    """Load schools from YAML files, returning those with empty image_urls."""
    from ruamel.yaml import YAML

    data_dir = Path(config.get("data_dir", "data/schools"))
    yaml = YAML()
    schools = []
    already_have = 0

    for path in sorted(data_dir.glob("*.yaml")):
        try:
            with path.open("r", encoding="utf-8") as f:
                data = dict(yaml.load(f) or {})
        except Exception as e:
            logger.warning("Failed to read %s: %s", path, e)
            continue

        existing_urls = data.get("image_urls", [])
        if existing_urls:
            already_have += 1
            continue

        schools.append({
            "school_id": data.get("school_id", path.stem),
            "name": data.get("name", ""),
            "website": data.get("website"),
            "address": data.get("address"),
        })

    logger.info("%d schools need photos, %d already have photos", len(schools), already_have)
    print(f"Schools needing photos: {len(schools)} (already have: {already_have})")
    return schools


async def run_photos(config: dict, force: bool = False) -> list[dict]:
    """Run photo enrichment for all schools missing photos."""
    client = get_client()
    # Process one school at a time for focused search
    batch_size = config.get("photo_batch_size", 1)
    semaphore_limit = config.get("photo_semaphore_limit", 4)

    if force:
        # If force, load all schools from index
        from ruamel.yaml import YAML
        data_dir = Path(config.get("data_dir", "data/schools"))
        yaml = YAML()
        schools = []
        for path in sorted(data_dir.glob("*.yaml")):
            try:
                with path.open("r", encoding="utf-8") as f:
                    data = dict(yaml.load(f) or {})
                schools.append({
                    "school_id": data.get("school_id", path.stem),
                    "name": data.get("name", ""),
                    "website": data.get("website"),
                    "address": data.get("address"),
                })
            except Exception:
                continue
        print(f"Force mode: searching photos for all {len(schools)} schools")
    else:
        schools = load_schools_needing_photos(config)

    if not schools:
        print("All schools already have photos!")
        return []

    sem = asyncio.Semaphore(semaphore_limit)
    batches = [schools[i:i + batch_size] for i in range(0, len(schools), batch_size)]
    logger.info(
        "Finding photos for %d schools in %d batches (semaphore=%d)",
        len(schools), len(batches), semaphore_limit,
    )

    batch_results = await asyncio.gather(
        *[enrich_photos_batch(b, sem, client, config, force) for b in batches],
        return_exceptions=True,
    )

    results = []
    for r in batch_results:
        if isinstance(r, Exception):
            logger.error("Photo batch failed: %s", r)
        else:
            results.extend(r)

    found = sum(1 for r in results if r.get("image_urls"))
    total_urls = sum(len(r.get("image_urls", [])) for r in results)
    print(f"Photo enrichment complete: {found}/{len(results)} schools got photos ({total_urls} URLs total)")
    return results


def merge_photos_into_yaml(results: list[dict], config: dict) -> int:
    """Merge photo results back into school YAML files. Returns count of updated files."""
    from ruamel.yaml import YAML

    data_dir = Path(config.get("data_dir", "data/schools"))
    yaml = YAML()
    yaml.default_flow_style = False
    yaml.width = 120
    yaml.allow_unicode = True
    updated = 0

    for result in results:
        sid = result.get("school_id")
        urls = result.get("image_urls", [])
        if not urls:
            continue

        path = data_dir / f"{sid}.yaml"
        if not path.exists():
            logger.warning("YAML file not found for %s", sid)
            continue

        try:
            with path.open("r", encoding="utf-8") as f:
                data = dict(yaml.load(f) or {})
        except Exception as e:
            logger.warning("Failed to read %s: %s", path, e)
            continue

        # Check pinned fields
        pinned = data.get("_pinned_fields", [])
        if "image_urls" in pinned:
            logger.info("Skipping %s: image_urls is pinned", sid)
            continue

        # Only update if currently empty or force
        existing = data.get("image_urls", [])
        if existing:
            continue

        data["image_urls"] = urls
        data["last_updated"] = date.today().isoformat()

        # Add to unverified_fields if not already there
        unverified = data.get("unverified_fields", [])
        if "image_urls" not in unverified:
            unverified.append("image_urls")
            data["unverified_fields"] = unverified

        # Add photo_search to data_sources
        sources = data.get("data_sources", [])
        if "photo_search" not in sources:
            sources.append("photo_search")
            data["data_sources"] = sources

        with path.open("w", encoding="utf-8") as f:
            yaml.dump(data, f)

        updated += 1
        logger.info("Updated photos for %s: %d URLs", sid, len(urls))

    print(f"Merged photos into {updated} school YAML files")
    return updated


async def run_photos_step(config: dict, force: bool = False) -> None:
    """Full photo enrichment step: find photos + merge into YAML."""
    results = await run_photos(config, force)
    if results:
        merge_photos_into_yaml(results, config)


if __name__ == "__main__":
    """Standalone: run photo enrichment."""
    import sys

    logging.basicConfig(level=logging.INFO, format="%(levelname)s %(message)s")

    from ruamel.yaml import YAML
    yaml = YAML()
    config_path = Path("pipeline/pipeline.yaml")
    config = dict(yaml.load(config_path) if config_path.exists() else {})

    force = "--force" in sys.argv
    asyncio.run(run_photos_step(config, force))
