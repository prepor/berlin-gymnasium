use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::detail_section::DetailSection;
use crate::components::rating_display::RatingDisplay;
use crate::models::School;
use crate::state::AppState;

/// Format an ISO date (YYYY-MM-DD) to German format (DD.MM.YYYY).
fn format_german_date(iso: &str) -> String {
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

/// Format a boolean Option as Ja/Nein/Keine Angabe.
fn bool_display(val: Option<bool>) -> &'static str {
    match val {
        Some(true) => "Ja",
        Some(false) => "Nein",
        None => "Keine Angabe",
    }
}

#[component]
pub fn DetailPage(id: String) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    let school = move || {
        state.schools.iter().find(|s| s.school_id == id).cloned()
    };

    view! {
        {move || {
            match school() {
                Some(s) => render_detail(s).into_any(),
                None => {
                    view! {
                        <main class="detail-page">
                            <a href="#/" class="back-link">"← Zurueck zur Liste"</a>
                            <h1>"Schule nicht gefunden"</h1>
                            <p>"Die angeforderte Schule konnte nicht gefunden werden."</p>
                            <a href="#/">"Zur Schuluebersicht"</a>
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
fn SchoolMap(lat: f64, lng: f64, name: String) -> impl IntoView {
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

fn render_detail(s: School) -> impl IntoView {
    // Pre-compute values for sections
    let has_profiles_or_languages = !s.profile.is_empty() || !s.languages.is_empty();
    let has_admission = s.admission_requirements.is_some();
    let has_ratings = !s.ratings.is_empty() || s.abitur_average.is_some();
    let has_open_day = s.open_day.is_some();

    // Hero section values
    let address_display = s
        .address
        .clone()
        .unwrap_or_else(|| "Keine Angabe".to_string());
    let traeger_label = match s.traeger.as_deref() {
        Some("privat") => "Privat",
        Some("oeffentlich") | Some("öffentlich") => "Oeffentlich",
        Some(_) => "Unbekannt",
        None => "Keine Angabe",
    };
    let student_teacher = match (s.student_count, s.teacher_count) {
        (Some(st), Some(te)) => format!("{} Schueler / {} Lehrkraefte", st, te),
        (Some(st), None) => format!("{} Schueler", st),
        (None, Some(te)) => format!("{} Lehrkraefte", te),
        (None, None) => "Keine Angabe".to_string(),
    };
    let ganztag_label = match s.ganztag {
        Some(true) => "Ganztagsschule",
        Some(false) => "Halbtagsschule",
        None => "Keine Angabe",
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
            let label = p.clone();
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
                .map(|g| format!("Klasse {}", g))
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
            .unwrap_or_else(|| "Keine Angabe".to_string());
        let ueberbucht = bool_display(adm.oversubscribed);
        let auswahl = adm
            .selection_criteria
            .clone()
            .unwrap_or_else(|| "Keine Angabe".to_string());
        let probe = bool_display(adm.probeunterricht);
        let aufnahme = bool_display(adm.entrance_test);
        let notes = adm.notes.clone();

        view! {
            <dl class="admission-list">
                <dt>"Notendurchschnitt"</dt>
                <dd>{noten}</dd>
                <dt>"Ueberbucht"</dt>
                <dd>{ueberbucht}</dd>
                <dt>"Auswahlverfahren"</dt>
                <dd>{auswahl}</dd>
                <dt>"Probeunterricht"</dt>
                <dd>{probe}</dd>
                <dt>"Aufnahmetest"</dt>
                <dd>{aufnahme}</dd>
                {notes.map(|n| view! {
                    <dt>"Hinweise"</dt>
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

    let abitur_view = s
        .abitur_average
        .map(|avg| view! { <p class="abitur-average">"Abiturdurchschnitt: "{format!("{:.2}", avg)}</p> });

    // Open day
    let open_day_view = s
        .open_day
        .as_ref()
        .map(|d| format_german_date(d));

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
    let last_updated_display = format_german_date(&s.last_updated);
    let sources_display = if s.data_sources.is_empty() {
        "Keine Angabe".to_string()
    } else {
        s.data_sources.join(", ")
    };
    let completeness_display = s
        .completeness_score
        .map(|c| format!("{:.0}% vollstaendig", c * 100.0))
        .unwrap_or_else(|| "Keine Angabe".to_string());

    view! {
        <main class="detail-page">
            // Back navigation
            <a href="javascript:history.back()" class="back-link">"← Zurueck zur Liste"</a>

            // Section 1: Hero
            <section class="detail-section detail-hero">
                <h1>{s.name.clone()}</h1>
                <p class="detail-district">{s.district.clone()}</p>
                <p class="detail-address">{address_display}</p>

                <div class="detail-badges">
                    <span class="badge badge-traeger">{traeger_label}</span>
                    {grundstaendig.then(|| view! {
                        <span class="badge badge-grundstaendig">"Grundstaendig (ab Klasse 5)"</span>
                    })}
                    <span class="badge badge-ganztag">{ganztag_label}</span>
                </div>

                <p class="detail-counts">{student_teacher}</p>

                <div class="detail-website-row">
                    {match website_view {
                        Some(v) => v.into_any(),
                        None => view! { <span class="keine-angabe">"Keine Angabe"</span> }.into_any(),
                    }}
                </div>
            </section>

            // Map
            {s.coords.as_ref().map(|c| {
                let name = s.name.clone();
                view! { <SchoolMap lat=c.lat lng=c.lng name=name /> }
            })}

            // Section 2: Profile & Languages
            <DetailSection title="Profil & Sprachen" empty={!has_profiles_or_languages}>
                <div class="profile-chips">
                    {profile_chips}
                </div>
                {if !s.languages.is_empty() {
                    view! {
                        <table class="languages-table">
                            <thead>
                                <tr>
                                    <th>"Sprache"</th>
                                    <th>"Ab Klasse"</th>
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
            <DetailSection title="Aufnahmebedingungen" empty={!has_admission}>
                {admission_view}
            </DetailSection>

            // Section 4: Ratings
            <DetailSection title="Bewertungen" empty={!has_ratings}>
                {rating_entries}
                {abitur_view}
            </DetailSection>

            // Section 5: Open Day
            <DetailSection title="Tag der offenen Tuer" empty={!has_open_day}>
                {open_day_view.map(|d| view! { <p>{d}</p> })}
            </DetailSection>

            // Section 6: Contact
            <DetailSection title="Kontakt" empty={s.phone.is_none() && s.email.is_none()}>
                <dl class="contact-list">
                    <dt>"Telefon"</dt>
                    <dd>
                        {match phone_view {
                            Some(v) => v.into_any(),
                            None => view! { <span class="keine-angabe">"Keine Angabe"</span> }.into_any(),
                        }}
                    </dd>
                    <dt>"E-Mail"</dt>
                    <dd>
                        {match email_view {
                            Some(v) => v.into_any(),
                            None => view! { <span class="keine-angabe">"Keine Angabe"</span> }.into_any(),
                        }}
                    </dd>
                </dl>
            </DetailSection>

            // Section 7: Data Provenance
            <DetailSection title="Datenherkunft" empty=false>
                <dl class="provenance-list">
                    <dt>"Letzte Aktualisierung"</dt>
                    <dd>{last_updated_display}</dd>
                    <dt>"Datenquellen"</dt>
                    <dd>{sources_display}</dd>
                    <dt>"Vollstaendigkeit"</dt>
                    <dd>{completeness_display}</dd>
                </dl>
            </DetailSection>
        </main>
    }
}
