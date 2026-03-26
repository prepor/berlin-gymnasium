use leptos::prelude::*;

use crate::models::{School, TravelTimes};

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
pub fn SchoolCard(
    school: School,
    #[prop(optional)] travel_times: Option<Signal<Option<TravelTimes>>>,
    #[prop(optional)] travel_loading: Option<Signal<bool>>,
) -> impl IntoView {
    let has_coords = school.coords.is_some();
    let href = format!("#/school/{}", school.school_id);
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

            // Travel time row: shown when travel_times signal exists
            {move || {
                let tt_signal = travel_times?;
                let loading_signal = travel_loading.unwrap_or(Signal::derive(|| false));
                let is_loading = loading_signal.get();

                if is_loading {
                    // Per D-21: loading spinner
                    Some(view! {
                        <div class="card-travel-times loading">
                            <span class="travel-spinner">"Berechne Fahrzeit..."</span>
                        </div>
                    }.into_any())
                } else {
                    let tt = tt_signal.get();
                    match tt {
                        Some(times) => {
                            // Per D-15: emoji + Xmin format
                            // Per D-22: show dash for None values
                            let walk = times.walk_minutes
                                .map(|m| format!("{} Min.", m))
                                .unwrap_or_else(|| "\u{2014}".to_string());
                            let bike = times.bike_minutes
                                .map(|m| format!("{} Min.", m))
                                .unwrap_or_else(|| "\u{2014}".to_string());
                            let car = times.car_minutes
                                .map(|m| format!("{} Min.", m))
                                .unwrap_or_else(|| "\u{2014}".to_string());
                            Some(view! {
                                <div class="card-travel-times">
                                    <span class="travel-mode walk" title="Zu Fuss">"\u{1F6B6} "{walk}</span>
                                    <span class="travel-mode bike" title="Fahrrad">"\u{1F6B2} "{bike}</span>
                                    <span class="travel-mode car" title="Auto">"\u{1F697} "{car}</span>
                                </div>
                            }.into_any())
                        }
                        None => {
                            // Per D-18: no coords or no route
                            if !has_coords {
                                Some(view! {
                                    <div class="card-travel-times no-data">
                                        <span class="keine-fahrzeit">"Keine Fahrzeit verfuegbar"</span>
                                    </div>
                                }.into_any())
                            } else {
                                None
                            }
                        }
                    }
                }
            }}

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
