use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::address_state::use_saved_address;
use crate::components::detail_section::DetailSection;
use crate::i18n::{profile_label, t, t_fmt, use_language, Language};
use crate::models::{School, TravelTimes};
use crate::services::routing::fetch_all_travel_times;
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
        "kunst" => "profile-kunst",
        "sport" | "sports" => "profile-sport",
        "bilingual_english" | "bilingual_french" | "bilingual_other" | "bilingual" => {
            "profile-bilingual"
        }
        "altsprachlich" => "profile-altsprachlich",
        "ib" => "profile-ib",
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

/// Travel info component: fetches and displays travel times from saved address.
#[component]
fn TravelInfo(school_id: String, school_lat: f64, school_lng: f64) -> impl IntoView {
    let lang = use_language();
    let saved_address = use_saved_address();
    let travel = RwSignal::new(Option::<TravelTimes>::None);
    let loading = RwSignal::new(false);

    Effect::new(move |_| {
        let addr = saved_address.get();
        if let Some(addr) = addr {
            loading.set(true);
            let sid = school_id.clone();
            let slat = school_lat;
            let slng = school_lng;
            wasm_bindgen_futures::spawn_local(async move {
                let coords = vec![(sid, slat, slng)];
                match fetch_all_travel_times(addr.lat, addr.lng, coords).await {
                    Ok(times) => {
                        let tt = times.values().next().cloned();
                        travel.set(tt);
                    }
                    Err(e) => {
                        log::error!("[TravelInfo] fetch error: {e}");
                        travel.set(None);
                    }
                }
                loading.set(false);
            });
        } else {
            travel.set(None);
            loading.set(false);
        }
    });

    move || {
        let l = lang.get();
        let addr = saved_address.get();
        let addr = addr?;
        Some(view! {
            <section class="detail-section travel-info-card">
                <div class="travel-info-header">
                    <span class="travel-info-icon">"\u{1F4CD}"</span>
                    <strong>{t("travel_from_address", l)}</strong>
                </div>
                <p class="travel-info-address">{addr.text}</p>
                {move || {
                    let l = lang.get();
                    if loading.get() {
                        view! {
                            <div class="travel-info-modes">
                                <span class="spinner spinner-lg"></span>
                                <span>{t("calculating_travel", l)}</span>
                            </div>
                        }.into_any()
                    } else if let Some(tt) = travel.get() {
                        view! {
                            <div class="travel-info-modes">
                                {tt.walk_minutes.map(|m| view! {
                                    <div class="travel-mode-card">
                                        <span class="travel-mode-icon">"\u{1F6B6}"</span>
                                        <span class="travel-mode-time">{m.to_string()} " Min."</span>
                                        <span class="travel-mode-label">{t("walk", l)}</span>
                                    </div>
                                })}
                                {tt.bike_minutes.map(|m| view! {
                                    <div class="travel-mode-card">
                                        <span class="travel-mode-icon">"\u{1F6B2}"</span>
                                        <span class="travel-mode-time">{m.to_string()} " Min."</span>
                                        <span class="travel-mode-label">{t("bike", l)}</span>
                                    </div>
                                })}
                                {tt.car_minutes.map(|m| view! {
                                    <div class="travel-mode-card">
                                        <span class="travel-mode-icon">"\u{1F697}"</span>
                                        <span class="travel-mode-time">{m.to_string()} " Min."</span>
                                        <span class="travel-mode-label">{t("car", l)}</span>
                                    </div>
                                })}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="travel-info-modes">
                                <span class="keine-angabe">{t("no_travel_time", l)}</span>
                            </div>
                        }.into_any()
                    }
                }}
            </section>
        })
    }
}

#[component]
pub fn DetailPage(id: String) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");
    let lang = use_language();

    let school_id = id.clone();
    let school = move || {
        state.schools.iter().find(|s| s.school_id == id).cloned()
    };

    view! {
        {move || {
            let l = lang.get();
            match school() {
                Some(s) => render_detail(s, l, school_id.clone()).into_any(),
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
        <div node_ref=map_ref class="detail-map" style="height: 220px; width: 100%; border-radius: 16px; margin: 0; border: 1px solid #C4C6CF;"></div>
    }
}

fn render_detail(s: School, lang: Language, school_id: String) -> impl IntoView {
    // Pre-compute values for sections
    let has_profiles_or_languages = !s.profile.is_empty() || !s.languages.is_empty();
    let has_admission = s.admission_requirements.is_some();
    let has_ratings = !s.ratings.is_empty() || s.abitur_average.is_some();
    let has_open_day = s.open_day.is_some();

    // Hero section values
    let address_display = s
        .address
        .clone()
        .unwrap_or_else(|| t("no_data", lang).to_string());
    let traeger_label = match s.traeger.as_deref() {
        Some("privat") => Some(t("private_school", lang)),
        Some("oeffentlich") | Some("öffentlich") => Some(t("public_school", lang)),
        Some(_) => Some(t("unknown", lang)),
        None => None,
    };
    let ganztag_label = match s.ganztag {
        Some(true) => Some(t("all_day_school", lang)),
        Some(false) => Some(t("half_day_school", lang)),
        None => None,
    };

    // Website
    let website_domain = s.website.as_ref().map(|url| extract_domain(url));
    let website_url = s.website.clone();
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

    // Hero photo: use first image or a placeholder
    let hero_photo_url = s.image_urls.first().cloned();

    // Stats bar values
    let stat_students = s.student_count.map(|n| n.to_string());
    let stat_teachers = s.teacher_count.map(|n| n.to_string());
    let stat_languages = if s.languages.is_empty() { None } else { Some(s.languages.len().to_string()) };
    let stat_grundstaendig = if grundstaendig { Some(t_fmt("grade_n", lang, &["5"])) } else { None };

    // Profile chips
    let profile_chips: Vec<_> = s
        .profile
        .iter()
        .filter_map(|p| {
            let label = profile_label(p, lang);
            if label.is_empty() {
                return None;
            }
            let class = format!("profile-badge {}", profile_color(p));
            let label = label.to_string();
            Some(view! { <span class={class}>{label}</span> })
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

    // Ratings: build horizontal rows with stars
    fn render_stars(score: f64, max: f64) -> Vec<leptos::tachys::view::any_view::AnyView> {
        let full_stars = (score / max * 5.0).round() as usize;
        (0..5).map(|i| {
            if i < full_stars {
                view! { <span class="star-filled">"\u{2605}"</span> }.into_any()
            } else {
                view! { <span class="star-empty">"\u{2605}"</span> }.into_any()
            }
        }).collect()
    }

    let mut rating_keys: Vec<String> = s.ratings.keys().cloned().collect();
    rating_keys.sort();
    let rating_entries: Vec<_> = rating_keys
        .into_iter()
        .filter_map(|key| {
            s.ratings.get(&key).map(|entry| {
                let source_name = key.split('_')
                    .map(|w| {
                        let mut c = w.chars();
                        match c.next() {
                            Some(ch) => format!("{}{}", ch.to_uppercase().collect::<String>(), c.collect::<String>()),
                            None => String::new(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let stars = entry.score.map(|s| render_stars(s, entry.scale_max));
                let score_text = entry.score.map(|s| format!("{:.1}", s));
                let review_text = entry.review_count.map(|n| format!("({} {})", n, t("reviews", lang)));
                view! {
                    <div class="rating-row">
                        <span class="rating-row-label">{source_name}</span>
                        <span class="rating-row-value">
                            {stars.map(|s| view! { <span class="rating-stars">{s}</span> })}
                            {score_text.map(|s| view! { <strong>{s}</strong> })}
                            {review_text.map(|r| view! { <span style="color:#64748b;font-size:0.8125rem">{r}</span> })}
                        </span>
                    </div>
                    <div class="rating-divider"></div>
                }
            })
        })
        .collect();

    let abitur_view = s.abitur_average.map(|avg| {
        let quality = if avg <= 1.5 { t("excellent", lang) } else if avg <= 2.0 { t("very_good", lang) } else if avg <= 2.5 { t("good", lang) } else { t("satisfactory", lang) };
        view! {
            <div class="rating-row">
                <span class="rating-row-label">{t("abitur_average", lang)}</span>
                <span class="rating-row-value">
                    <span class="abitur-value">{format!("{:.2}", avg)}</span>
                    <span class="abitur-quality">{quality}</span>
                </span>
            </div>
            <div class="rating-divider"></div>
        }
    });

    // Demand row for ratings section
    let demand_view = s.admission_requirements.as_ref().and_then(|adm| {
        adm.demand_ratio.or_else(|| {
            match (adm.first_choices, adm.places) {
                (Some(fc), Some(pl)) if pl > 0 => Some(fc as f64 / pl as f64),
                _ => None,
            }
        }).map(|ratio| {
            let (label, class) = if ratio > 1.0 {
                (t("demand_high", lang), "demand-badge")
            } else {
                (t("demand_low", lang), "demand-badge demand-low")
            };
            view! {
                <div class="rating-row">
                    <span class="rating-row-label">{t("demand_label", lang)}</span>
                    <span class="rating-row-value">
                        <span class={class}>{label}</span>
                    </span>
                </div>
            }
        })
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

    // Remaining photos (skip the first one used in hero)
    let remaining_photos: Vec<_> = s.image_urls.iter().skip(1).cloned().collect();
    let has_remaining_photos = !remaining_photos.is_empty();

    view! {
        <main class="detail-page">
            // Hero Banner with photo, gradient overlay, and content
            <div class="detail-hero-banner">
                {hero_photo_url.map(|url| view! {
                    <img class="hero-photo" src={url} alt="" />
                })}
                <div class="hero-gradient"></div>
                <a href="javascript:history.back()" class="hero-back-pill">
                    {t("back_to_list", lang)}
                </a>
                <div class="hero-content">
                    <h1 class="hero-title">{s.name.clone()}</h1>
                    <p class="hero-district">{s.district.clone()}</p>
                    <p class="hero-address">{address_display}</p>
                    <div class="hero-badges">
                        {traeger_label.map(|label| view! {
                            <span class="hero-badge">{label}</span>
                        })}
                        {grundstaendig.then(|| view! {
                            <span class="hero-badge">{t("grundstaendig", lang)}</span>
                        })}
                        {ganztag_label.map(|label| view! {
                            <span class="hero-badge">{label}</span>
                        })}
                    </div>
                </div>
            </div>

            // Stats Bar
            <div class="detail-stats-bar">
                {stat_students.map(|n| view! {
                    <div class="stat-item">
                        <span class="stat-value">{n}</span>
                        <span class="stat-label">{t("students_label", lang)}</span>
                    </div>
                    <div class="stat-divider"></div>
                })}
                {stat_teachers.map(|n| view! {
                    <div class="stat-item">
                        <span class="stat-value">{n}</span>
                        <span class="stat-label">{t("teachers_label", lang)}</span>
                    </div>
                    <div class="stat-divider"></div>
                })}
                {stat_languages.map(|n| view! {
                    <div class="stat-item">
                        <span class="stat-value">{n}</span>
                        <span class="stat-label">{t("languages_label", lang)}</span>
                    </div>
                    <div class="stat-divider"></div>
                })}
                {stat_grundstaendig.map(|g| view! {
                    <div class="stat-item">
                        <span class="stat-value">{g}</span>
                        <span class="stat-label">{t("grundstaendig", lang)}</span>
                    </div>
                    <div class="stat-divider"></div>
                })}
                {website_url.as_ref().map(|url| {
                    let href = url.clone();
                    let domain = website_domain.clone().unwrap_or_default();
                    view! {
                        <a href={href} target="_blank" rel="noopener noreferrer" class="stat-item" style="text-decoration:none">
                            <span class="stat-value" style="font-size:1.25rem">"🌐"</span>
                            <span class="stat-label">{domain}</span>
                        </a>
                    }
                })}
            </div>

            // Photos (remaining, after hero photo)
            {has_remaining_photos.then(|| {
                let photos: Vec<_> = remaining_photos.iter().map(|url| {
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

            // Travel info from saved address
            {s.coords.as_ref().map(|c| {
                let sid = school_id.clone();
                view! { <TravelInfo school_id=sid school_lat=c.lat school_lng=c.lng /> }
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
                {abitur_view}
                {rating_entries}
                {demand_view}
            </DetailSection>

            // Section 5: Open Day
            <DetailSection title=t("open_day", lang) empty={!has_open_day}>
                {open_day_view.map(|d| view! { <p>{d}</p> })}
            </DetailSection>

            // Section 6: Contact
            <DetailSection title=t("contact", lang) empty={s.phone.is_none() && s.email.is_none() && s.website.is_none()}>
                {phone_view.map(|v| view! {
                    <div class="contact-row">
                        <span class="contact-icon">"\u{1F4DE}"</span>
                        {v}
                    </div>
                })}
                {email_view.map(|v| view! {
                    <div class="contact-row">
                        <span class="contact-icon">"\u{2709}\u{FE0F}"</span>
                        {v}
                    </div>
                })}
                {website_view.map(|v| view! {
                    <div class="contact-row">
                        <span class="contact-icon">"\u{1F310}"</span>
                        {v}
                    </div>
                })}
            </DetailSection>

            // Section 7: Data Provenance (muted styling)
            <section class="detail-section provenance">
                <h2>{t("data_provenance", lang)}</h2>
                <dl class="provenance-list">
                    <dt>{t("last_updated", lang)}</dt>
                    <dd>{last_updated_display}</dd>
                    <dt>{t("data_sources", lang)}</dt>
                    <dd>{sources_display}</dd>
                    <dt>{t("completeness_label", lang)}</dt>
                    <dd>{completeness_display}</dd>
                </dl>
            </section>
        </main>
    }
}
