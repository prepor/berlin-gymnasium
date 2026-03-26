use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Embedded JSON produced by build.rs from data/schools/*.yaml
const SCHOOLS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/schools.json"));

/// Load all schools from the embedded JSON data.
pub fn load_schools() -> Vec<School> {
    serde_json::from_str(SCHOOLS_JSON).expect("embedded schools.json must be valid")
}

/// A single Berlin Gymnasium. Mirrors pipeline/models.py SchoolRecord.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct School {
    pub school_id: String,
    pub name: String,
    pub district: String,
    pub last_updated: String,

    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub coords: Option<Coords>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub traeger: Option<String>,
    #[serde(default)]
    pub student_count: Option<u32>,
    #[serde(default)]
    pub teacher_count: Option<u32>,

    #[serde(default)]
    pub accepts_after_4th_grade: Option<bool>,
    #[serde(default)]
    pub profile: Vec<String>,
    #[serde(default)]
    pub ganztag: Option<bool>,
    #[serde(default)]
    pub languages: Vec<LanguageEntry>,
    #[serde(default)]
    pub open_day: Option<String>,
    #[serde(default)]
    pub admission_requirements: Option<AdmissionRequirements>,
    #[serde(default)]
    pub abitur_average: Option<f64>,
    #[serde(default)]
    pub image_urls: Vec<String>,
    #[serde(default)]
    pub social_media: HashMap<String, String>,

    #[serde(default)]
    pub ratings: HashMap<String, RatingEntry>,

    #[serde(default)]
    pub completeness_score: Option<f64>,
    #[serde(default)]
    pub field_confidence: HashMap<String, String>,

    #[serde(default)]
    pub data_sources: Vec<String>,
    #[serde(default)]
    pub unverified_fields: Vec<String>,

    #[serde(default, alias = "_pinned_fields")]
    pub pinned_fields: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coords {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LanguageEntry {
    pub name: String,
    #[serde(default)]
    pub from_grade: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AdmissionRequirements {
    #[serde(default)]
    pub notendurchschnitt: Option<f64>,
    #[serde(default)]
    pub oversubscribed: Option<bool>,
    #[serde(default)]
    pub selection_criteria: Option<String>,
    #[serde(default)]
    pub probeunterricht: Option<bool>,
    #[serde(default)]
    pub entrance_test: Option<bool>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RatingEntry {
    pub source: String,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default = "default_scale_min")]
    pub scale_min: f64,
    #[serde(default = "default_scale_max")]
    pub scale_max: f64,
    #[serde(default)]
    pub review_count: Option<u32>,
    pub retrieved: String,
}

fn default_scale_min() -> f64 {
    1.0
}
fn default_scale_max() -> f64 {
    5.0
}
