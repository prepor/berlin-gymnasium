"""
Berlin Gymnasien pipeline — Pydantic schema models.

This is the single source of truth for the YAML schema. All pipeline modules
import from here. Do not define field structures anywhere else.

Field rules:
- Only school_id, name, district, last_updated are required.
- All other fields are Optional or have empty defaults.
- Missing fields are absent in YAML, not null (produces cleaner diffs).
- _pinned_fields: writer never overwrites fields listed here (manual override, Pitfall 8).
- unverified_fields: agent-only data not cross-referenced with structured sources (D-19).
"""
from __future__ import annotations

from typing import Literal, Optional
from pydantic import BaseModel, Field


class RatingEntry(BaseModel):
    """A single rating from one source. Never normalize scores — store raw. (Pitfall 6)"""
    source: str                          # e.g. "tagesspiegel_abitur", "schul_ranking_de"
    score: Optional[float] = None
    scale_min: float = 1.0
    scale_max: float = 5.0
    review_count: Optional[int] = None
    retrieved: str                       # ISO date e.g. "2026-03-26"


class LanguageEntry(BaseModel):
    """A foreign language offered by the school."""
    name: str                            # e.g. "Englisch", "Französisch", "Latein"
    from_grade: Optional[int] = None     # e.g. 5 or 7


class AdmissionRequirements(BaseModel):
    """School-specific admission criteria. DATA-11. All fields optional (best-effort)."""
    notendurchschnitt: Optional[float] = None   # GPA threshold e.g. 2.2
    oversubscribed: Optional[bool] = None       # True if school is typically oversubscribed
    selection_criteria: Optional[str] = None    # Free text description
    probeunterricht: Optional[bool] = None      # Trial lesson required for admission
    entrance_test: Optional[bool] = None        # Entrance exam required
    notes: Optional[str] = None                 # Additional context


class SchoolRecord(BaseModel):
    """
    Canonical record for one Berlin Gymnasium. One YAML file per school.
    File path: data/schools/{school_id}.yaml

    Provenance (DATA-05):
    - data_sources: list of source identifiers used for this record
    - last_updated: ISO date of last pipeline run that updated this record
    - unverified_fields: fields sourced only from agent (not cross-referenced)
    - field_confidence: per-field confidence level from agent (D-15)
    """

    # --- Required fields ---
    school_id: str                              # BSN from WFS e.g. "01k01"
    name: str                                   # Full school name
    district: str                               # Berlin Bezirk e.g. "Mitte"
    last_updated: str                           # ISO date e.g. "2026-03-26" (DATA-05)

    # --- Structured fields (from WFS / Eckdaten XLSX) ---
    address: Optional[str] = None               # Full address string
    coords: Optional[dict] = None               # {"lat": float, "lng": float}
    website: Optional[str] = None               # School website URL
    phone: Optional[str] = None
    email: Optional[str] = None
    traeger: Optional[str] = None               # "öffentlich" or "privat"
    student_count: Optional[int] = None         # From Eckdaten XLSX
    teacher_count: Optional[int] = None         # From Eckdaten XLSX

    # --- Agent-enriched fields ---
    accepts_after_4th_grade: Optional[bool] = None      # DATA-10: grundständig flag
    profile: list[str] = Field(default_factory=list)    # e.g. ["MINT", "bilingual_english"]
    ganztag: Optional[bool] = None                       # Full-day school flag
    languages: list[LanguageEntry] = Field(default_factory=list)   # Foreign languages
    open_day: Optional[str] = None                       # DATA-12: ISO date of next open day
    admission_requirements: Optional[AdmissionRequirements] = None # DATA-11
    abitur_average: Optional[float] = None               # D-24: from Tagesspiegel
    image_urls: list[str] = Field(default_factory=list)  # D-14: school photo URLs
    social_media: dict[str, str] = Field(default_factory=dict)  # D-14: {"instagram": url}

    # --- Ratings (DATA-09) ---
    # Keyed by source identifier. Never use bare numeric score — always full RatingEntry.
    ratings: dict[str, RatingEntry] = Field(default_factory=dict)

    # --- Quality tracking (D-15) ---
    completeness_score: Optional[float] = None          # % fields filled, 0.0–1.0
    field_confidence: dict[str, Literal["high", "medium", "low"]] = Field(default_factory=dict)

    # --- Provenance (DATA-05) ---
    data_sources: list[str] = Field(default_factory=list)  # e.g. ["wfs_berlin", "agent_research"]
    unverified_fields: list[str] = Field(default_factory=list)  # D-19: agent-only fields

    # --- Manual override support (Pitfall 8 / open question 4) ---
    # List any field names here; writer will never overwrite them during re-runs.
    pinned_fields: list[str] = Field(default_factory=list, alias="_pinned_fields")

    model_config = {"populate_by_name": True}

    def compute_completeness(self) -> float:
        """
        Compute and store completeness_score as fraction of non-empty fields.
        Excludes meta-fields: school_id, last_updated, data_sources, unverified_fields,
        field_confidence, completeness_score, pinned_fields.
        """
        trackable_fields = [
            "address", "coords", "website", "phone", "email", "traeger",
            "student_count", "teacher_count", "accepts_after_4th_grade",
            "profile", "ganztag", "languages", "open_day", "admission_requirements",
            "abitur_average", "image_urls", "ratings",
        ]
        filled = sum(
            1 for f in trackable_fields
            if (v := getattr(self, f, None)) is not None and v != [] and v != {}
        )
        self.completeness_score = round(filled / len(trackable_fields), 3)
        return self.completeness_score
