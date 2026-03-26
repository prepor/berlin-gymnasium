use leptos::prelude::*;

use crate::i18n::{t, t_fmt, use_language};
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

/// Format an ISO date string (YYYY-MM-DD) to DD.MM.YYYY.
fn format_date(iso: &str) -> String {
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
    let lang = use_language();
    let source_name = humanize_source(&source_key);
    let score = entry.score;
    let scale_max = entry.scale_max;
    let review_count = entry.review_count;
    let retrieved = entry.retrieved.clone();

    let score_display = move || {
        match score {
            Some(s) => format!("{:.1} / {:.1}", s, scale_max),
            None => t("no_rating", lang.get()).to_string(),
        }
    };

    let review_text = move || {
        let l = lang.get();
        review_count.map(|n| t_fmt("n_reviews", l, &[&n.to_string()]))
    };

    let retrieved_text = {
        let r = retrieved.clone();
        move || t_fmt("as_of", lang.get(), &[&format_date(&r)])
    };

    view! {
        <div class="rating-entry">
            <strong>{source_name}</strong>
            <span class="rating-score">{" "}{score_display}</span>
            {move || review_text().map(|t| view! { <span class="rating-reviews">{" "}{t}</span> })}
            <div class="rating-retrieved">{retrieved_text}</div>
        </div>
    }
}
