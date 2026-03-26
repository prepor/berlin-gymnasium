"""
pipeline/writer.py — Write validated SchoolRecord objects to YAML files.

Pinned fields (Pitfall 8): fields in _pinned_fields in existing YAML are never overwritten.
Changelog: deepdiff between prior and new YAML; append to data/CHANGELOG.md (DATA-06).
Conflicts: regenerate data/conflicts.md from all_conflicts (D-20).
YAML: ruamel.yaml for comment-preserving round-trip writes.
"""
from __future__ import annotations

import logging
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from deepdiff import DeepDiff
from ruamel.yaml import YAML

from pipeline.models import SchoolRecord

logger = logging.getLogger(__name__)


def _yaml_instance() -> YAML:
    yaml = YAML()
    yaml.default_flow_style = False
    yaml.width = 120
    yaml.allow_unicode = True
    return yaml


def read_existing_yaml(school_id: str, data_dir: str) -> dict:
    """Read existing school YAML if it exists. Returns empty dict if not found."""
    path = Path(data_dir) / f"{school_id}.yaml"
    if not path.exists():
        return {}
    yaml = _yaml_instance()
    try:
        with path.open("r", encoding="utf-8") as f:
            return dict(yaml.load(f) or {})
    except Exception as e:
        logger.warning("Failed to read existing YAML for %s: %s", school_id, e)
        return {}


def apply_pinned_fields(new_record_dict: dict, existing: dict) -> dict:
    """
    Apply _pinned_fields from existing YAML to new_record_dict.
    Pinned fields are never overwritten by the pipeline (Pitfall 8).
    """
    pinned = existing.get("_pinned_fields", [])
    if not pinned:
        return new_record_dict
    for field in pinned:
        if field in existing:
            new_record_dict[field] = existing[field]
            logger.debug("Pinned field %s preserved for %s", field, new_record_dict.get("school_id"))
    # Keep _pinned_fields in the output YAML
    new_record_dict["_pinned_fields"] = pinned
    return new_record_dict


def compute_changelog_entry(school_id: str, name: str, prior: dict, new: dict) -> str:
    """
    Compute human-readable changelog entry for one school using deepdiff.
    Returns empty string if no changes.
    """
    # Exclude meta fields that always change
    exclude_paths = {"root['last_updated']", "root['completeness_score']"}
    diff = DeepDiff(prior, new, ignore_order=True, exclude_paths=exclude_paths, view="text")
    if not diff:
        return ""

    lines = [f"### {school_id} — {name}"]
    for change_type, changes in diff.items():
        if change_type == "values_changed":
            for path, change in changes.items():
                lines.append(f"- `{path}`: {change['old_value']!r} → {change['new_value']!r}")
        elif change_type == "dictionary_item_added":
            for path in (changes if hasattr(changes, "__iter__") else [changes]):
                lines.append(f"- `{path}`: added")
        elif change_type == "dictionary_item_removed":
            for path in (changes if hasattr(changes, "__iter__") else [changes]):
                lines.append(f"- `{path}`: removed")
        elif change_type == "iterable_item_added":
            for path, val in (changes.items() if hasattr(changes, "items") else []):
                lines.append(f"- `{path}`: + {val!r}")
        elif change_type == "iterable_item_removed":
            for path, val in (changes.items() if hasattr(changes, "items") else []):
                lines.append(f"- `{path}`: - {val!r}")
    if len(lines) == 1:
        return ""  # Only header, no actual changes (shouldn't happen but safety)
    return "\n".join(lines)


def write_school_yaml(record: SchoolRecord, data_dir: str) -> tuple[bool, str]:
    """
    Write one school's YAML. Returns (changed: bool, changelog_entry: str).
    Respects _pinned_fields from existing YAML.
    """
    path = Path(data_dir) / f"{record.school_id}.yaml"
    path.parent.mkdir(parents=True, exist_ok=True)

    existing = read_existing_yaml(record.school_id, data_dir)

    # Convert record to dict for writing (exclude None values for clean YAML)
    record_dict = record.model_dump(exclude_none=True, by_alias=True)

    # Apply pinned fields (Pitfall 8)
    record_dict = apply_pinned_fields(record_dict, existing)

    # Compute changelog entry
    changelog_entry = compute_changelog_entry(
        record.school_id, record.name, existing, record_dict
    )
    changed = bool(changelog_entry) or not existing  # new record also counts as changed

    # Write YAML
    yaml = _yaml_instance()
    with path.open("w", encoding="utf-8") as f:
        yaml.dump(record_dict, f)

    return changed, changelog_entry


def write_all(records: list[SchoolRecord], data_dir: str) -> dict:
    """
    Write all school YAML files. Appends to data/CHANGELOG.md.
    Returns stats dict: {written, changed, unchanged, changelog_entries}.
    """
    written = 0
    changed = 0
    unchanged = 0
    changelog_entries = []

    for record in records:
        was_changed, entry = write_school_yaml(record, data_dir)
        written += 1
        if was_changed:
            changed += 1
            if entry:
                changelog_entries.append(entry)
        else:
            unchanged += 1

    # Append to CHANGELOG.md (never overwrite — DATA-06)
    if changelog_entries:
        changelog_path = Path(data_dir).parent / "CHANGELOG.md"
        ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        header = f"\n## {ts}\n"
        body = "\n\n".join(changelog_entries)
        with changelog_path.open("a", encoding="utf-8") as f:
            f.write(header + body + "\n")
        logger.info("Appended %d changelog entries to %s", len(changelog_entries), changelog_path)

    # D-28: minimal stdout
    print(f"Write complete: {written} files written ({changed} changed, {unchanged} unchanged)")
    if changelog_entries:
        print(f"Changelog: {len(changelog_entries)} schools updated → data/CHANGELOG.md")

    return {
        "written": written,
        "changed": changed,
        "unchanged": unchanged,
        "changelog_entries": len(changelog_entries),
    }


def write_conflicts(all_conflicts: list[dict], data_dir: str) -> None:
    """
    Regenerate data/conflicts.md from collected conflicts (D-20).
    Overwrites on each run — shows current state of disagreements.
    """
    conflicts_path = Path(data_dir).parent / "conflicts.md"
    if not all_conflicts:
        conflicts_path.write_text("# Data Conflicts\n\nNo conflicts detected in last run.\n", encoding="utf-8")
        return

    from datetime import date
    lines = [f"# Data Conflicts — {date.today().isoformat()}\n"]
    lines.append("Fields where agent data contradicted structured source. Structured value was used (D-17).\n")

    # Group by school_id
    by_school: dict[str, list[dict]] = {}
    for c in all_conflicts:
        sid = c.get("school_id", "unknown")
        by_school.setdefault(sid, []).append(c)

    for sid, conflicts in sorted(by_school.items()):
        lines.append(f"\n## {sid}")
        for c in conflicts:
            lines.append(
                f"- Field `{c['field']}`: structured={c['structured_value']!r}, "
                f"agent={c['agent_value']!r} → using structured"
            )

    conflicts_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    logger.info("Written %s (%d conflicts across %d schools)", conflicts_path, len(all_conflicts), len(by_school))
    print(f"Conflicts: {len(all_conflicts)} disagreements in {len(by_school)} schools → data/conflicts.md")
