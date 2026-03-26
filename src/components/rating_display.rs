use leptos::prelude::*;

use crate::models::RatingEntry;

/// Humanize a rating source key: replace underscores with spaces, capitalize each word.
fn humanize_source(key: &str) -> String {
    key.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{}{}", upper, chars.collect::<String>())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format an ISO date string (YYYY-MM-DD) to German format (DD.MM.YYYY).
fn format_german_date(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() == 3 {
        format!("{}.{}.{}", parts[2], parts[1], parts[0])
    } else {
        iso.to_string()
    }
}

/// Displays a single rating entry with source name, score, review count, and retrieval date.
#[component]
pub fn RatingDisplay(source_key: String, entry: RatingEntry) -> impl IntoView {
    let source_name = humanize_source(&source_key);

    let score_display = match entry.score {
        Some(score) => format!("{:.1} / {:.1}", score, entry.scale_max),
        None => "Keine Bewertung".to_string(),
    };

    let review_text = entry
        .review_count
        .map(|n| format!("({} Bewertungen)", n));

    let retrieved_text = format!("Stand: {}", format_german_date(&entry.retrieved));

    view! {
        <div class="rating-entry">
            <strong>{source_name}</strong>
            <span class="rating-score">{" "}{score_display}</span>
            {review_text.map(|t| view! { <span class="rating-reviews">{" "}{t}</span> })}
            <div class="rating-retrieved">{retrieved_text}</div>
        </div>
    }
}
