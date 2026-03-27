pub mod academic;
pub mod data;
pub mod districts;
pub mod grade4;
pub mod hype_quality;
pub mod private_public;
pub mod supply_demand;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::hash_router::navigate_hash;
use crate::i18n::{use_language, Language};
use crate::models::School;

use academic::AcademicExcellence;
use districts::DistrictPowerRankings;
use grade4::Grade4Advantage;
use hype_quality::HypeQuality;
use private_public::PrivatePublic;
use supply_demand::SupplyDemand;

/// Metadata for each infographic (title keys, action, link).
struct InfographicMeta {
    title_de: &'static str,
    title_en: &'static str,
    action_de: &'static str,
    action_en: &'static str,
    link: &'static str,
}

const INFOGRAPHICS: &[InfographicMeta] = &[
    InfographicMeta {
        title_de: "Akademische Exzellenz",
        title_en: "Academic Excellence",
        action_de: "Alle Schulen anzeigen",
        action_en: "View all schools",
        link: "/",
    },
    InfographicMeta {
        title_de: "Angebot vs. Nachfrage",
        title_en: "Supply vs Demand",
        action_de: "Nach Schülerzahl sortieren",
        action_en: "Sort by student count",
        link: "/?sort=students",
    },
    InfographicMeta {
        title_de: "Hype = Qualität?",
        title_en: "Does Hype = Quality?",
        action_de: "Alle Schulen anzeigen",
        action_en: "View all schools",
        link: "/",
    },
    InfographicMeta {
        title_de: "Privat vs. Öffentlich",
        title_en: "Private vs Public",
        action_de: "Alle Schulen anzeigen",
        action_en: "View all schools",
        link: "/",
    },
    InfographicMeta {
        title_de: "Vorteil ab Klasse 5",
        title_en: "The 4th Grade Advantage",
        action_de: "Grundständige Schulen",
        action_en: "View early-entry schools",
        link: "/?grundstaendig=ja",
    },
    InfographicMeta {
        title_de: "Bezirks-Ranking",
        title_en: "District Power Rankings",
        action_de: "Karte anzeigen",
        action_en: "View on map",
        link: "/?view=map",
    },
];

/// Render an infographic card by index (takes owned Vec to avoid lifetime issues).
fn render_card(idx: usize, schools: Vec<School>) -> impl IntoView {
    match idx {
        0 => view! { <AcademicExcellence schools=schools /> }.into_any(),
        1 => view! { <SupplyDemand schools=schools /> }.into_any(),
        2 => view! { <HypeQuality schools=schools /> }.into_any(),
        3 => view! { <PrivatePublic schools=schools /> }.into_any(),
        4 => view! { <Grade4Advantage schools=schools /> }.into_any(),
        5 => view! { <DistrictPowerRankings schools=schools /> }.into_any(),
        _ => view! { <div></div> }.into_any(),
    }
}

/// Horizontal strip of infographic thumbnails for the page header.
/// Each thumbnail is a CSS-scaled real component. Clicking opens a modal.
#[component]
pub fn InfographicThumbnails(schools: Vec<School>) -> impl IntoView {
    let open_index = RwSignal::new(Option::<usize>::None);
    let schools_for_modal = schools.clone();

    view! {
        <div class="ig-thumbnails">
            {(0..INFOGRAPHICS.len()).map(|i| {
                let s = schools.clone();
                view! {
                    <button
                        class="ig-thumb"
                        on:click=move |_| open_index.set(Some(i))
                        title=INFOGRAPHICS[i].title_en
                    >
                        <div class="ig-thumb-scale">
                            {render_card(i, s)}
                        </div>
                    </button>
                }
            }).collect_view()}
        </div>
        <InfographicModal open_index=open_index schools=schools_for_modal />
    }
}

/// Full-screen modal showing an infographic with navigation.
#[component]
fn InfographicModal(
    open_index: RwSignal<Option<usize>>,
    schools: Vec<School>,
) -> impl IntoView {
    let lang = use_language();
    let overlay_ref = NodeRef::<leptos::html::Div>::new();

    // Focus the overlay when the modal opens so it captures keyboard events
    Effect::new(move |_| {
        if open_index.get().is_some() {
            let el_ref = overlay_ref;
            let cb = Closure::once(move || {
                if let Some(el) = el_ref.get() {
                    let _ = el.focus();
                }
            });
            if let Some(w) = web_sys::window() {
                let _ = w.set_timeout_with_callback(cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }
    });

    let total = INFOGRAPHICS.len();

    view! {
        {move || {
            let l = lang.get();
            let schools = schools.clone();
            open_index.get().map(move |idx| {
                let meta = &INFOGRAPHICS[idx];
                let title = if l == Language::De { meta.title_de } else { meta.title_en };
                let action = if l == Language::De { meta.action_de } else { meta.action_en };
                let link = meta.link.to_string();
                let schools_for_card = schools.clone();

                view! {
                    <div
                        class="ig-modal-overlay"
                        node_ref=overlay_ref
                        tabindex="-1"
                        on:click=move |_| open_index.set(None)
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            match ev.key().as_str() {
                                "Escape" => open_index.set(None),
                                "ArrowLeft" => {
                                    if idx > 0 { open_index.set(Some(idx - 1)); }
                                }
                                "ArrowRight" => {
                                    if idx < total - 1 { open_index.set(Some(idx + 1)); }
                                }
                                _ => {}
                            }
                        }
                    >
                        <div class="ig-modal" on:click=move |ev: leptos::ev::MouseEvent| ev.stop_propagation()>
                            <div class="ig-modal-top">
                                <span class="ig-modal-counter">
                                    {format!("{} / {}", idx + 1, total)}
                                </span>
                                <div class="ig-modal-dots">
                                    {(0..total).map(|i| {
                                        let cls = if i == idx { "ig-dot active" } else { "ig-dot" };
                                        view! {
                                            <button class=cls on:click=move |ev: leptos::ev::MouseEvent| {
                                                ev.stop_propagation();
                                                open_index.set(Some(i));
                                            }></button>
                                        }
                                    }).collect_view()}
                                </div>
                                <button
                                    class="ig-modal-close"
                                    on:click=move |_| open_index.set(None)
                                >
                                    "\u{2715}"
                                </button>
                            </div>

                            <div class="ig-modal-body">
                                {render_card(idx, schools_for_card)}
                            </div>

                            <div class="ig-modal-bottom">
                                <span class="ig-modal-title">{title}</span>
                                <a
                                    class="ig-modal-cta"
                                    href="#"
                                    on:click=move |ev: leptos::ev::MouseEvent| {
                                        ev.prevent_default();
                                        open_index.set(None);
                                        navigate_hash(&link, false);
                                    }
                                >
                                    {action} " \u{2192}"
                                </a>
                            </div>

                            {(idx > 0).then(|| view! {
                                <button
                                    class="ig-nav ig-nav-prev"
                                    on:click=move |ev: leptos::ev::MouseEvent| {
                                        ev.stop_propagation();
                                        open_index.set(Some(idx - 1));
                                    }
                                >
                                    "\u{2039}"
                                </button>
                            })}
                            {(idx < total - 1).then(|| view! {
                                <button
                                    class="ig-nav ig-nav-next"
                                    on:click=move |ev: leptos::ev::MouseEvent| {
                                        ev.stop_propagation();
                                        open_index.set(Some(idx + 1));
                                    }
                                >
                                    "\u{203A}"
                                </button>
                            })}
                        </div>
                        <span class="ig-modal-esc">"ESC"</span>
                    </div>
                }
            })
        }}
    }
}
