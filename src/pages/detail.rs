use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::detail_section::DetailSection;
use crate::components::rating_display::RatingDisplay;
use crate::i18n::{profile_label, t, t_fmt, use_language, Language};
use crate::models::School;
use crate::state::AppState;

/// Format an ISO date (YYYY-MM-DD) to DD.MM.YYYY.
fn format_date(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() == 3 {
        format!("{}.{}.{}", parts[2], parts[1], parts[0])
    } else {
        iso.to_string()
    }
}

/// Extract just the domain from a URL for display purposes.
fn extract_domain(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .trim_end_matches('/')
        .to_string()
}

/// Map a profile keyword to a CSS color class.
fn profile_color(profile: &str) -> &'static str {
    match profile.to_lowercase().as_str() {
        "mint" => "profile-mint",
        "musik" | "music" => "profile-musik",
        "sport" | "sports" => "profile-sport",
        "bilingual" => "profile-bilingual",
        "altsprachlich" => "profile-altsprachlich",
        _ => "profile-other",
    }
}

/// Format a boolean Option as Yes/No/No data.
fn bool_display(val: Option<bool>, lang: Language) -> &'static str {
    match val {
        Some(true) => t("yes", lang),
        Some(false) => t("no", lang),
        None => t("no_data", lang),
    }
}

#[component]
pub fn DetailPage(id: String) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");
    let lang = use_language();

    let school = move || {
        state.schools.iter().find(|s| s.school_id == id).cloned()
    };

    view! {
        {move || {
            let l = lang.get();
            match school() {
                Some(s) => render_detail(s, l).into_any(),
                None => {
                    view! {
                        <main class="detail-page">
                            <a href="#/" class="back-link">{t("back_to_list", l)}</a>
                            <h1>{t("school_not_found", l)}</h1>
                            <p>{t("school_not_found_desc", l)}</p>
                            <a href="#/">{t("to_overview", l)}</a>
                        </main>
                    }
                        .into_any()
                }
            }
        }}
    }
}

/// Small Leaflet map showing the school's location.
#[component]
fn SchoolMap(lat: f64, lng: f64, _name: String) -> impl IntoView {
    let map_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if let Some(el) = map_ref.get() {
            let html_el: &web_sys::HtmlElement = el.as_ref();

            // Create map centered on school
            let lat_lng = leaflet::LatLng::new(lat, lng);
            let map_opts = leaflet::MapOptions::default();
            let map = leaflet::Map::new_with_element(html_el, &map_opts)
                .expect("Leaflet Map::new_with_element failed");
            map.set_view(&lat_lng, 15.0);

            // Add OSM tile layer
            leaflet::TileLayer::new("https://tile.openstreetmap.org/{z}/{x}/{y}.png")
                .add_to(&map);

            // Add marker for the school
            let marker_opts = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&marker_opts, &"radius".into(), &JsValue::from_f64(10.0));
            let _ = js_sys::Reflect::set(&marker_opts, &"fillColor".into(), &JsValue::from_str("#3b82f6"));
            let _ = js_sys::Reflect::set(&marker_opts, &"color".into(), &JsValue::from_str("#fff"));
            let _ = js_sys::Reflect::set(&marker_opts, &"weight".into(), &JsValue::from_f64(2.0));
            let _ = js_sys::Reflect::set(&marker_opts, &"fillOpacity".into(), &JsValue::from_f64(0.9));

            let marker = leaflet::CircleMarker::new_with_options(&lat_lng, &marker_opts.into());
            marker.add_to(&map);

            // Invalidate size after render (Leaflet needs this when container size changes)
            let map_clone = map.clone();
            let cb = Closure::once(move || {
                map_clone.invalidate_size(false);
            });
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    100,
                )
                .ok();
            cb.forget();
        }
    });

    view! {
        <div node_ref=map_ref class="detail-map" style="height: 250px; width: 100%; border-radius: 8px; margin: 1rem 0;"></div>
    }
}

fn render_detail(s: School, lang: Language) -> impl IntoView {
    // Pre-compute values for sections
    let has_profiles_or_languages = !s.profile.is_empty() || !s.languages.is_empty();
    let has_admission = s.admission_requirements.is_some();
    let has_ratings = !s.ratings.is_empty() || s.abitur_average.is_some();
    let has_open_day = s.open_day.is_some();
    let has_photos = !s.image_urls.is_empty();

    // Hero section values
    let address_display = s
        .address
        .clone()
        .unwrap_or_else(|| t("no_data", lang).to_string());
    let traeger_label = match s.traeger.as_deref() {
        Some("privat") => t("private_school", lang),
        Some("oeffentlich") | Some("öffentlich") => t("public_school", lang),
        Some(_) => t("unknown", lang),
        None => t("no_data", lang),
    };
    let student_teacher = match (s.student_count, s.teacher_count) {
        (Some(st), Some(te)) => {
            t_fmt("n_students_n_teachers", lang, &[&st.to_string(), &te.to_string()])
        }
        (Some(st), None) => t_fmt("n_students", lang, &[&st.to_string()]),
        (None, Some(te)) => t_fmt("n_teachers", lang, &[&te.to_string()]),
        (None, None) => t("no_data", lang).to_string(),
    };
    let ganztag_label = match s.ganztag {
        Some(true) => t("all_day_school", lang),
        Some(false) => t("half_day_school", lang),
        None => t("no_data", lang),
    };

    // Website
    let website_view = s.website.clone().map(|url| {
        let domain = extract_domain(&url);
        view! {
            <a href={url} target="_blank" rel="noopener noreferrer" class="detail-website">
                {domain}
            </a>
        }
    });

    // Grundstaendig badge
    let grundstaendig = s.accepts_after_4th_grade == Some(true);

    // Profile chips
    let profile_chips: Vec<_> = s
        .profile
        .iter()
        .map(|p| {
            let class = format!("profile-badge {}", profile_color(p));
            let label = profile_label(p, lang).to_string();
            view! { <span class={class}>{label}</span> }
        })
        .collect();

    // Languages table rows
    let language_rows: Vec<_> = s
        .languages
        .iter()
        .map(|l| {
            let grade_text = l
                .from_grade
                .map(|g| t_fmt("grade_n", lang, &[&g.to_string()]))
                .unwrap_or_else(|| "-".to_string());
            let name = l.name.clone();
            view! {
                <tr>
                    <td>{name}</td>
                    <td>{grade_text}</td>
                </tr>
            }
        })
        .collect();

    // Admission requirements
    let admission_view = s.admission_requirements.as_ref().map(|adm| {
        let noten = adm
            .notendurchschnitt
            .map(|v| format!("{:.1}", v))
            .unwrap_or_else(|| t("no_data", lang).to_string());
        let ueberbucht = bool_display(adm.oversubscribed, lang);
        let auswahl = adm
            .selection_criteria
            .clone()
            .unwrap_or_else(|| t("no_data", lang).to_string());
        let probe = bool_display(adm.probeunterricht, lang);
        let aufnahme = bool_display(adm.entrance_test, lang);
        let notes = adm.notes.clone();
        let demand_view = match (adm.first_choices, adm.places) {
            (Some(fc), Some(pl)) => {
                let ratio = adm.demand_ratio.unwrap_or(fc as f64 / pl as f64);
                let status = if ratio > 1.0 {
                    t("demand_oversubscribed", lang)
                } else {
                    t("demand_undersubscribed", lang)
                };
                let color = if ratio > 1.5 {
                    "#dc2626"
                } else if ratio > 1.0 {
                    "#ca8a04"
                } else {
                    "#16a34a"
                };
                Some(view! {
                    <dt>{t("demand_label", lang)}</dt>
                    <dd>
                        <span style=format!("font-weight:600;color:{}", color)>
                            {format!("{:.1}x", ratio)}
                        </span>
                        " ("
                        {t_fmt("demand_ratio_fmt", lang, &[&fc.to_string(), &pl.to_string()])}
                        " \u{2014} " {status} ")"
                    </dd>
                })
            }
            _ => None,
        };

        view! {
            <dl class="admission-list">
                <dt>{t("grade_average", lang)}</dt>
                <dd>{noten}</dd>
                {demand_view}
                <dt>{t("oversubscribed", lang)}</dt>
                <dd>{ueberbucht}</dd>
                <dt>{t("selection_process", lang)}</dt>
                <dd>{auswahl}</dd>
                <dt>{t("trial_class", lang)}</dt>
                <dd>{probe}</dd>
                <dt>{t("entrance_test", lang)}</dt>
                <dd>{aufnahme}</dd>
                {notes.map(|n| view! {
                    <dt>{t("notes", lang)}</dt>
                    <dd>{n}</dd>
                })}
            </dl>
        }
    });

    // Ratings entries
    let mut rating_keys: Vec<String> = s.ratings.keys().cloned().collect();
    rating_keys.sort();
    let rating_entries: Vec<_> = rating_keys
        .into_iter()
        .filter_map(|key| {
            s.ratings.get(&key).map(|entry| {
                let k = key.clone();
                let e = entry.clone();
                view! { <RatingDisplay source_key={k} entry={e} /> }
            })
        })
        .collect();

    let abitur_view = s.abitur_average.map(|avg| {
        let avg_text = t_fmt("abitur_avg", lang, &[&format!("{:.2}", avg)]);
        let pass_text = s.abitur_pass_rate.map(|r| t_fmt("abitur_pass_rate", lang, &[&format!("{:.1}", r)]));
        let count_text = s.abitur_student_count.map(|n| t_fmt("abitur_students", lang, &[&n.to_string()]));
        let avg_color = if avg <= 1.8 { "#16a34a" } else if avg <= 2.3 { "#ca8a04" } else { "#6b7280" };
        view! {
            <div class="abitur-stats">
                <span class="abitur-average" style=format!("color:{};font-size:1.5rem;font-weight:700", avg_color)>
                    {format!("{:.2}", avg)}
                </span>
                <div class="abitur-details">
                    <p>{avg_text}</p>
                    {pass_text.map(|t| view! { <p>{t}</p> })}
                    {count_text.map(|t| view! { <p>{t}</p> })}
                </div>
            </div>
        }
    });

    // Open day
    let open_day_view = s
        .open_day
        .as_ref()
        .map(|d| format_date(d));

    // Contact
    let phone_view = s.phone.clone().map(|p| {
        let href = format!("tel:{}", p);
        let display = p.clone();
        view! { <a href={href}>{display}</a> }
    });

    let email_view = s.email.clone().map(|e| {
        let href = format!("mailto:{}", e);
        let display = e.clone();
        view! { <a href={href}>{display}</a> }
    });

    // Data provenance
    let last_updated_display = format_date(&s.last_updated);
    let sources_display = if s.data_sources.is_empty() {
        t("no_data", lang).to_string()
    } else {
        s.data_sources.join(", ")
    };
    let completeness_display = s
        .completeness_score
        .map(|c| t_fmt("pct_complete", lang, &[&format!("{:.0}", c * 100.0)]))
        .unwrap_or_else(|| t("no_data", lang).to_string());

    let no_data_label = t("no_data", lang);

    view! {
        <main class="detail-page">
            // Back navigation
            <a href="javascript:history.back()" class="back-link">{t("back_to_list", lang)}</a>

            // Section 1: Hero
            <section class="detail-section detail-hero">
                <h1>{s.name.clone()}</h1>
                <p class="detail-district">{s.district.clone()}</p>
                <p class="detail-address">{address_display}</p>

                <div class="detail-badges">
                    <span class="badge badge-traeger">{traeger_label}</span>
                    {grundstaendig.then(|| view! {
                        <span class="badge badge-grundstaendig">{t("grundstaendig", lang)}</span>
                    })}
                    <span class="badge badge-ganztag">{ganztag_label}</span>
                </div>

                <p class="detail-counts">{student_teacher}</p>

                <div class="detail-website-row">
                    {match website_view {
                        Some(v) => v.into_any(),
                        None => view! { <span class="keine-angabe">{no_data_label}</span> }.into_any(),
                    }}
                </div>
            </section>

            // Photos
            {has_photos.then(|| {
                let photos: Vec<_> = s.image_urls.iter().map(|url| {
                    let src = url.clone();
                    view! {
                        <img class="detail-photo" src=src alt="" loading="lazy" />
                    }
                }).collect();
                view! {
                    <div class="detail-photos">
                        {photos}
                    </div>
                }
            })}

            // Map
            {s.coords.as_ref().map(|c| {
                let name = s.name.clone();
                view! { <SchoolMap lat=c.lat lng=c.lng _name=name /> }
            })}

            // Section 2: Profile & Languages
            <DetailSection title=t("profile_languages", lang) empty={!has_profiles_or_languages}>
                <div class="profile-chips">
                    {profile_chips}
                </div>
                {if !s.languages.is_empty() {
                    view! {
                        <table class="languages-table">
                            <thead>
                                <tr>
                                    <th>{t("language_col", lang)}</th>
                                    <th>{t("from_grade", lang)}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {language_rows}
                            </tbody>
                        </table>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </DetailSection>

            // Section 3: Admission Requirements
            <DetailSection title=t("admission", lang) empty={!has_admission}>
                {admission_view}
            </DetailSection>

            // Section 4: Ratings
            <DetailSection title=t("ratings", lang) empty={!has_ratings}>
                {rating_entries}
                {abitur_view}
            </DetailSection>

            // Section 5: Open Day
            <DetailSection title=t("open_day", lang) empty={!has_open_day}>
                {open_day_view.map(|d| view! { <p>{d}</p> })}
            </DetailSection>

            // Section 6: Contact
            <DetailSection title=t("contact", lang) empty={s.phone.is_none() && s.email.is_none()}>
                <dl class="contact-list">
                    <dt>{t("telephone", lang)}</dt>
                    <dd>
                        {match phone_view {
                            Some(v) => v.into_any(),
                            None => view! { <span class="keine-angabe">{no_data_label}</span> }.into_any(),
                        }}
                    </dd>
                    <dt>{t("email_label", lang)}</dt>
                    <dd>
                        {match email_view {
                            Some(v) => v.into_any(),
                            None => view! { <span class="keine-angabe">{no_data_label}</span> }.into_any(),
                        }}
                    </dd>
                </dl>
            </DetailSection>

            // Section 7: Data Provenance
            <DetailSection title=t("data_provenance", lang) empty=false>
                <dl class="provenance-list">
                    <dt>{t("last_updated", lang)}</dt>
                    <dd>{last_updated_display}</dd>
                    <dt>{t("data_sources", lang)}</dt>
                    <dd>{sources_display}</dd>
                    <dt>{t("completeness_label", lang)}</dt>
                    <dd>{completeness_display}</dd>
                </dl>
            </DetailSection>
        </main>
    }
}
