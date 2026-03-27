use leptos::prelude::*;

use crate::i18n::{t, t_fmt, profile_label, use_language};
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

/// A single school rendered as a clickable card.
#[component]
pub fn SchoolCard(
    school: School,
    #[prop(optional)] travel_times: Option<Signal<Option<TravelTimes>>>,
    #[prop(optional)] travel_loading: Option<Signal<bool>>,
) -> impl IntoView {
    let lang = use_language();
    let has_coords = school.coords.is_some();
    let href = format!("#/school/{}", school.school_id);
    let name = school.name.clone();
    let district = school.district.clone();
    let profiles = school.profile.clone();
    let grundstaendig = school.accepts_after_4th_grade == Some(true);
    let thumbnail = school.image_urls.first().cloned();
    let student_count = school.student_count;
    let teacher_count = school.teacher_count;
    let abitur_avg = school.abitur_average;
    let abitur_pass = school.abitur_pass_rate;
    let schulen_de_stars = school.ratings.get("schulen_de").and_then(|r| r.score);
    let demand_ratio = school.admission_requirements.as_ref().and_then(|a| a.demand_ratio);
    let completeness = school.completeness_score.unwrap_or(0.0);
    let completeness_pct = (completeness * 100.0) as u32;

    let student_text = move || {
        let l = lang.get();
        match (student_count, teacher_count) {
            (Some(s), Some(t)) => t_fmt("n_students_n_teachers", l, &[&s.to_string(), &t.to_string()]),
            (Some(s), None) => t_fmt("n_students", l, &[&s.to_string()]),
            (None, Some(t)) => t_fmt("n_teachers", l, &[&t.to_string()]),
            (None, None) => t("no_data", l).to_string(),
        }
    };
    let completeness_text =
        move || t_fmt("pct_complete", lang.get(), &[&completeness_pct.to_string()]);

    view! {
        <a class="school-card" href=href>
            {thumbnail.map(|url| view! {
                <div class="card-thumbnail">
                    <img src=url alt="" loading="lazy" />
                </div>
            })}
            <h3 class="card-title">{name}</h3>
            <p class="card-district">{district}</p>

            <div class="card-badges">
                {move || {
                    let l = lang.get();
                    profiles
                        .iter()
                        .map(|p| {
                            let color = profile_color(p);
                            let label = profile_label(p, l).to_string();
                            let style = format!(
                                "background:{};color:#fff;padding:2px 8px;border-radius:12px;font-size:0.75rem;font-weight:600;display:inline-block;margin:2px",
                                color,
                            );
                            view! { <span class="profile-badge" style=style>{label}</span> }
                        })
                        .collect::<Vec<_>>()
                }}
                {move || {
                    grundstaendig.then(|| view! {
                        <span
                            class="grundstaendig-badge"
                            style="background:#0d9488;color:#fff;padding:2px 8px;border-radius:12px;font-size:0.75rem;font-weight:600;display:inline-block;margin:2px"
                        >
                            {t("from_grade_5", lang.get())}
                        </span>
                    })
                }}
            </div>

            <div class="card-metrics">
                {abitur_avg.map(|avg| {
                    let color = if avg <= 1.8 { "#16a34a" } else if avg <= 2.3 { "#ca8a04" } else { "#6b7280" };
                    view! {
                        <span class="metric" style=format!("color:{}", color) title=move || t("abitur_avg", lang.get())>
                            {format!("{:.1}", avg)}
                        </span>
                    }
                })}
                {abitur_pass.map(|rate| {
                    let color = if rate >= 98.0 { "#16a34a" } else if rate >= 90.0 { "#ca8a04" } else { "#dc2626" };
                    view! {
                        <span class="metric" style=format!("color:{}", color) title=move || t("abitur_pass_rate", lang.get())>
                            {format!("{:.0}%", rate)}
                        </span>
                    }
                })}
                {schulen_de_stars.map(|stars| {
                    let full = stars as u32;
                    let half = if stars - (full as f64) >= 0.5 { 1 } else { 0 };
                    let empty = 5 - full - half;
                    view! {
                        <span class="metric metric-stars" title="schulen.de">
                            {"\u{2605}".repeat(full as usize)}
                            {if half > 0 { "\u{00BD}" } else { "" }}
                            {"\u{2606}".repeat(empty as usize)}
                        </span>
                    }
                })}
                {demand_ratio.map(|ratio| {
                    let color = if ratio > 1.5 { "#dc2626" } else if ratio > 1.0 { "#ca8a04" } else { "#16a34a" };
                    let label = if ratio > 1.0 {
                        format!("{:.1}x", ratio)
                    } else {
                        "\u{2714}".to_string()
                    };
                    view! {
                        <span class="metric" style=format!("color:{}", color) title=move || t("demand_label", lang.get())>
                            {label}
                        </span>
                    }
                })}
            </div>

            <div class="card-meta">
                <span class="card-students">{student_text}</span>
            </div>

            // Travel time row: shown when travel_times signal exists
            {move || {
                let l = lang.get();
                let tt_signal = travel_times?;
                let loading_signal = travel_loading.unwrap_or(Signal::derive(|| false));
                let is_loading = loading_signal.get();

                if is_loading {
                    Some(view! {
                        <div class="card-travel-times loading">
                            <span class="travel-spinner">{t("calculating_travel", l)}</span>
                        </div>
                    }.into_any())
                } else {
                    let tt = tt_signal.get();
                    match tt {
                        Some(times) => {
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
                                    <span class="travel-mode walk" title=t("on_foot", l)>"\u{1F6B6} "{walk}</span>
                                    <span class="travel-mode bike" title=t("bicycle", l)>"\u{1F6B2} "{bike}</span>
                                    <span class="travel-mode car" title=t("car", l)>"\u{1F697} "{car}</span>
                                </div>
                            }.into_any())
                        }
                        None => {
                            if !has_coords {
                                Some(view! {
                                    <div class="card-travel-times no-data">
                                        <span class="keine-fahrzeit">{t("no_travel_time", l)}</span>
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
