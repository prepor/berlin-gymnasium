"""
pipeline/validate.py — Merge structured + agent data, run Pydantic validation.

Merge rule (D-17): Structured source always wins.
Agent-only fields: added to unverified_fields (D-19).
Conflicts: collected in a separate list for conflicts.md (D-20).
Missing fields: set to null (D-18).
"""
from __future__ import annotations

import logging
from datetime import date
from typing import Any

from pydantic import ValidationError

from pipeline.models import SchoolRecord

logger = logging.getLogger(__name__)

# Fields that come from structured sources (WFS / Eckdaten XLSX).
# These always win over agent data when both are present.
STRUCTURED_FIELDS = {
    "school_id", "name", "district", "address", "coords",
    "website", "phone", "email", "traeger", "student_count", "teacher_count",
}

# Fields that are agent-only (no structured source exists).
# These are added to unverified_fields if the agent provides them.
AGENT_ONLY_FIELDS = {
    "accepts_after_4th_grade", "profile", "ganztag", "languages", "open_day",
    "admission_requirements", "ratings", "abitur_average", "image_urls", "social_media",
}


def merge_and_validate(
    structured: dict,
    agent: dict,
) -> tuple[SchoolRecord | None, list[dict]]:
    """
    Merge structured and agent dicts into a validated SchoolRecord.

    Returns:
        (SchoolRecord, conflicts): Record is None if validation fails.
        conflicts: list of {school_id, field, structured_value, agent_value} dicts.
    """
    conflicts = []
    merged = dict(structured)  # start with structured fields
    merged["last_updated"] = date.today().isoformat()
    merged.setdefault("data_sources", [])

    # Track provenance
    if "wfs_berlin" not in merged["data_sources"]:
        merged["data_sources"].append("wfs_berlin")

    unverified: list[str] = []

    for field, agent_value in agent.items():
        if field in ("school_id", "enriched_at", "data"):
            continue  # skip cache metadata fields
        if agent_value is None:
            continue  # agent found nothing — leave as absent

        if field in STRUCTURED_FIELDS:
            # D-17: structured source wins
            structured_value = structured.get(field)
            if structured_value is not None and structured_value != agent_value:
                conflicts.append({
                    "school_id": structured.get("school_id"),
                    "field": field,
                    "structured_value": structured_value,
                    "agent_value": agent_value,
                })
            # Do not overwrite — structured value stays in merged
        elif field in AGENT_ONLY_FIELDS:
            merged[field] = agent_value
            unverified.append(field)  # D-19: agent-only
        else:
            # Unknown field from agent — still accept, flag as unverified
            merged[field] = agent_value
            unverified.append(field)

    if unverified and "agent_research" not in merged["data_sources"]:
        merged["data_sources"].append("agent_research")

    # D-19: record which fields are agent-only
    merged["unverified_fields"] = list(set(unverified))

    # Compute completeness score (D-15)
    # Done post-validation via the method, but we do a best-effort pre-validation
    # (SchoolRecord.compute_completeness() called after validation below)

    # Validate with Pydantic (DATA-08)
    try:
        record = SchoolRecord.model_validate(merged)
        record.compute_completeness()
        return record, conflicts
    except ValidationError as e:
        logger.error(
            "Validation failed for school %s: %s",
            merged.get("school_id", "unknown"), e
        )
        return None, conflicts


def merge_all(
    structured_list: list[dict],
    agent_list: list[dict],
) -> tuple[list[SchoolRecord], list[dict]]:
    """
    Merge all structured and agent records. Returns (valid_records, all_conflicts).
    Skips invalid records (logs error) without stopping the pipeline.
    """
    # Build agent lookup by school_id
    agent_by_id = {r.get("school_id"): r for r in agent_list if r.get("school_id")}

    records: list[SchoolRecord] = []
    all_conflicts: list[dict] = []
    failed = 0

    for structured in structured_list:
        sid = structured.get("school_id")
        agent = agent_by_id.get(sid, {})
        record, conflicts = merge_and_validate(structured, agent)
        all_conflicts.extend(conflicts)

        if record is not None:
            records.append(record)
        else:
            failed += 1

    logger.info(
        "Merge complete: %d valid, %d failed validation, %d conflicts",
        len(records), failed, len(all_conflicts),
    )
    if failed > 0:
        logger.warning("%d schools failed Pydantic validation and will not be written", failed)

    return records, all_conflicts
