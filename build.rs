//! Build script: reads all data/schools/*.yaml files, converts to a single JSON array,
//! and writes to OUT_DIR/schools.json for embedding via include_str! at compile time.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Coords {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize)]
struct LanguageEntry {
    name: String,
    #[serde(default)]
    from_grade: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct AdmissionRequirements {
    #[serde(default)]
    notendurchschnitt: Option<f64>,
    #[serde(default)]
    oversubscribed: Option<bool>,
    #[serde(default)]
    selection_criteria: Option<String>,
    #[serde(default)]
    probeunterricht: Option<bool>,
    #[serde(default)]
    entrance_test: Option<bool>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RatingEntry {
    source: String,
    #[serde(default)]
    score: Option<f64>,
    #[serde(default = "default_scale_min")]
    scale_min: f64,
    #[serde(default = "default_scale_max")]
    scale_max: f64,
    #[serde(default)]
    review_count: Option<u32>,
    retrieved: String,
}

fn default_scale_min() -> f64 {
    1.0
}
fn default_scale_max() -> f64 {
    5.0
}

/// Mirrors pipeline/models.py SchoolRecord — all fields included.
#[derive(Serialize, Deserialize)]
struct SchoolRecord {
    // Required fields
    school_id: String,
    name: String,
    district: String,
    last_updated: String,

    // Structured fields (from WFS / Eckdaten XLSX)
    #[serde(default)]
    address: Option<String>,
    #[serde(default)]
    coords: Option<Coords>,
    #[serde(default)]
    website: Option<String>,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    traeger: Option<String>,
    #[serde(default)]
    student_count: Option<u32>,
    #[serde(default)]
    teacher_count: Option<u32>,

    // Agent-enriched fields
    #[serde(default)]
    accepts_after_4th_grade: Option<bool>,
    #[serde(default)]
    profile: Vec<String>,
    #[serde(default)]
    ganztag: Option<bool>,
    #[serde(default)]
    languages: Vec<LanguageEntry>,
    #[serde(default)]
    open_day: Option<String>,
    #[serde(default)]
    admission_requirements: Option<AdmissionRequirements>,
    #[serde(default)]
    abitur_average: Option<f64>,
    #[serde(default)]
    image_urls: Vec<String>,
    #[serde(default)]
    social_media: HashMap<String, String>,

    // Ratings (keyed by source identifier)
    #[serde(default)]
    ratings: HashMap<String, RatingEntry>,

    // Quality tracking
    #[serde(default)]
    completeness_score: Option<f64>,
    #[serde(default)]
    field_confidence: HashMap<String, String>,

    // Provenance
    #[serde(default)]
    data_sources: Vec<String>,
    #[serde(default)]
    unverified_fields: Vec<String>,

    // Manual override support (aliased as _pinned_fields in YAML)
    #[serde(default, alias = "_pinned_fields")]
    pinned_fields: Vec<String>,
}

fn main() {
    let schools_dir = Path::new("data/schools");
    let mut schools: Vec<SchoolRecord> = Vec::new();

    let entries = fs::read_dir(schools_dir).expect("data/schools/ directory must exist");
    for entry in entries {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "yaml") {
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
            let school: SchoolRecord = serde_saphyr::from_str(&content)
                .unwrap_or_else(|e| panic!("failed to parse {}: {}", path.display(), e));
            schools.push(school);
        }
    }

    // Sort alphabetically by name for stable output
    schools.sort_by(|a, b| a.name.cmp(&b.name));

    let json = serde_json::to_string(&schools).expect("failed to serialize schools to JSON");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    fs::write(Path::new(&out_dir).join("schools.json"), json)
        .expect("failed to write schools.json");

    // Re-run if any YAML file changes
    println!("cargo:rerun-if-changed=data/schools/");
}
