use leptos::prelude::*;

use crate::models::School;

/// Returns the badge color for a given profile type.
fn profile_color(profile: &str) -> &'static str {
    match profile {
        "MINT" => "#22c55e",
        "bilingual_english" | "bilingual_french" => "#f97316",
        "altsprachlich" => "#ef4444",
        "music" => "#a855f7",
        "sports" => "#3b82f6",
        _ => "#6b7280",
    }
}

/// Returns a human-readable German label for a profile type.
fn profile_label(profile: &str) -> &str {
    match profile {
        "MINT" => "MINT",
        "bilingual_english" => "Bilingual EN",
        "bilingual_french" => "Bilingual FR",
        "altsprachlich" => "Altsprachlich",
        "music" => "Musik",
        "sports" => "Sport",
        "other" => "Sonstiges",
        _ => profile,
    }
}

/// A single school rendered as a clickable card.
#[component]
pub fn SchoolCard(school: School) -> impl IntoView {
    let href = format!("/school/{}", school.school_id);
    let name = school.name.clone();
    let district = school.district.clone();
    let profiles = school.profile.clone();
    let grundstaendig = school.accepts_after_4th_grade == Some(true);
    let student_text = match school.student_count {
        Some(n) => format!("{} Schueler", n),
        None => "Keine Angabe".to_string(),
    };
    let completeness = school.completeness_score.unwrap_or(0.0);
    let completeness_pct = (completeness * 100.0) as u32;
    let completeness_text = format!("{}% vollstaendig", completeness_pct);

    view! {
        <a class="school-card" href=href>
            <h3 class="card-title">{name}</h3>
            <p class="card-district">{district}</p>

            <div class="card-badges">
                {profiles
                    .into_iter()
                    .map(|p| {
                        let color = profile_color(&p);
                        let label = profile_label(&p).to_string();
                        let style = format!(
                            "background:{};color:#fff;padding:2px 8px;border-radius:12px;font-size:0.75rem;font-weight:600;display:inline-block;margin:2px",
                            color,
                        );
                        view! { <span class="profile-badge" style=style>{label}</span> }
                    })
                    .collect::<Vec<_>>()}
                {if grundstaendig {
                    Some(
                        view! {
                            <span
                                class="grundstaendig-badge"
                                style="background:#0d9488;color:#fff;padding:2px 8px;border-radius:12px;font-size:0.75rem;font-weight:600;display:inline-block;margin:2px"
                            >
                                "ab Klasse 5"
                            </span>
                        },
                    )
                } else {
                    None
                }}
            </div>

            <div class="card-meta">
                <span class="card-students">{student_text}</span>
            </div>

            <div class="card-completeness">
                <div class="completeness-bar-bg">
                    <div
                        class="completeness-bar-fill"
                        style=format!("width:{}%", completeness_pct)
                    ></div>
                </div>
                <span class="completeness-text">{completeness_text}</span>
            </div>
        </a>
    }
}
