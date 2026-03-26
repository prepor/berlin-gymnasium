#!/usr/bin/env python3
"""
Merge cached enrichment data INTO existing YAML files without losing existing data.

For each school:
1. Read existing YAML (preserves all previously enriched fields)
2. Read cache JSON (new enrichment from agent research)
3. For each field in cache: only overwrite if the existing field is empty/missing
4. Write updated YAML
"""
import json
import logging
import sys
from datetime import date
from pathlib import Path

from ruamel.yaml import YAML

logging.basicConfig(level=logging.INFO, format="%(levelname)s %(message)s")
logger = logging.getLogger(__name__)

ENRICHMENT_FIELDS = {
    "accepts_after_4th_grade", "profile", "ganztag", "languages", "open_day",
    "admission_requirements", "abitur_average", "social_media", "ratings",
    "unverified_fields", "field_confidence",
}

TRACKABLE_FIELDS = [
    "address", "coords", "website", "phone", "email", "traeger",
    "student_count", "teacher_count", "accepts_after_4th_grade",
    "profile", "ganztag", "languages", "open_day", "admission_requirements",
    "abitur_average", "image_urls", "ratings",
]


def is_empty(val):
    """Check if a value is effectively empty."""
    if val is None:
        return True
    if isinstance(val, (list, dict)) and len(val) == 0:
        return True
    return False


def compute_completeness(data: dict) -> float:
    filled = sum(1 for f in TRACKABLE_FIELDS if not is_empty(data.get(f)))
    return round(filled / len(TRACKABLE_FIELDS), 3)


def merge_school(yaml_data: dict, cache_data: dict) -> tuple[dict, list[str]]:
    """
    Merge cache enrichment into existing YAML data.
    Only fills empty fields — never overwrites existing data.
    Returns (merged_data, list_of_changed_fields).
    """
    changed = []
    agent_data = cache_data.get("data", {})

    for field, value in agent_data.items():
        if field not in ENRICHMENT_FIELDS:
            continue
        if is_empty(value):
            continue

        existing = yaml_data.get(field)

        # Only fill if existing is empty
        if is_empty(existing):
            yaml_data[field] = value
            changed.append(field)
        elif field == "social_media" and isinstance(existing, dict) and isinstance(value, dict):
            # Merge social media dicts (add new platforms)
            for platform, url in value.items():
                if platform not in existing:
                    existing[platform] = url
                    changed.append(f"social_media.{platform}")
        elif field == "ratings" and isinstance(existing, dict) and isinstance(value, dict):
            # Merge ratings (add new sources)
            for source, entry in value.items():
                if source not in existing:
                    existing[source] = entry
                    changed.append(f"ratings.{source}")
        elif field == "field_confidence" and isinstance(existing, dict) and isinstance(value, dict):
            for k, v in value.items():
                if k not in existing:
                    existing[k] = v
        elif field == "unverified_fields" and isinstance(existing, list) and isinstance(value, list):
            for v in value:
                if v not in existing:
                    existing.append(v)

    # Update provenance
    if changed:
        sources = yaml_data.get("data_sources", [])
        if "agent_research" not in sources:
            sources.append("agent_research")
            yaml_data["data_sources"] = sources
        yaml_data["last_updated"] = date.today().isoformat()

    # Recompute completeness
    yaml_data["completeness_score"] = compute_completeness(yaml_data)

    return yaml_data, changed


def main():
    yaml = YAML()
    yaml.default_flow_style = False
    yaml.width = 120
    yaml.allow_unicode = True

    schools_dir = Path("data/schools")
    cache_dir = Path("pipeline/cache")

    total = 0
    updated = 0
    fields_added = 0

    for yaml_path in sorted(schools_dir.glob("*.yaml")):
        school_id = yaml_path.stem
        cache_path = cache_dir / f"{school_id}.json"
        total += 1

        if not cache_path.exists():
            continue

        # Read existing YAML
        with yaml_path.open("r", encoding="utf-8") as f:
            yaml_data = dict(yaml.load(f) or {})

        # Read cache
        try:
            cache_data = json.loads(cache_path.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError) as e:
            logger.warning("Bad cache for %s: %s", school_id, e)
            continue

        # Merge
        merged, changed = merge_school(yaml_data, cache_data)

        if changed:
            with yaml_path.open("w", encoding="utf-8") as f:
                yaml.dump(merged, f)
            updated += 1
            fields_added += len(changed)
            logger.info("%s: added %s", school_id, ", ".join(changed))

    print(f"\nMerge complete: {updated}/{total} schools updated, {fields_added} fields added")


if __name__ == "__main__":
    main()
