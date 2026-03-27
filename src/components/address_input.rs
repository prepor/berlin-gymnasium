use leptos::prelude::*;
use leptos_use::use_debounce_fn_with_arg;
use wasm_bindgen_futures::spawn_local;

use crate::address_state::{clear_address, save_address, use_saved_address};
use crate::i18n::{t, use_language};
use crate::services::geocoding::{geocode_address, PhotonFeature};

/// Address input component with debounced geocoding and suggestion dropdown.
/// Lets the user type an address, see up to 5 suggestions, and select one.
/// Persists the selected address to localStorage via address_state.
#[component]
pub fn AddressInput(
    on_address_selected: Callback<(f64, f64)>,
    on_address_cleared: Callback<()>,
    initial_coords: Signal<Option<(f64, f64)>>,
    travel_loading: Signal<bool>,
) -> impl IntoView {
    let lang = use_language();
    let saved_address = use_saved_address();

    // Initialize input value from saved address if available
    let initial_text = saved_address
        .get_untracked()
        .map(|a| a.text.clone())
        .unwrap_or_default();
    let input_value = RwSignal::new(initial_text);

    let suggestions = RwSignal::new(Vec::<PhotonFeature>::new());
    let show_suggestions = RwSignal::new(false);
    let is_loading = RwSignal::new(false);
    // Generation counter: incremented on submit to invalidate any pending debounced request
    let geocode_epoch = RwSignal::new(0u64);

    // Debounced geocoding: fires 500ms after user stops typing (per D-19)
    let debounced_geocode = use_debounce_fn_with_arg(
        move |query: String| {
            if query.len() < 3 {
                suggestions.set(vec![]);
                show_suggestions.set(false);
                return;
            }
            let epoch = geocode_epoch.get();
            is_loading.set(true);
            spawn_local(async move {
                match geocode_address(&query).await {
                    Ok(results) => {
                        // Only apply if no submit has fired since we started
                        if geocode_epoch.get() == epoch {
                            suggestions.set(results);
                            show_suggestions.set(true);
                        }
                    }
                    Err(e) => {
                        if geocode_epoch.get() == epoch {
                            log::error!("[AddressInput] geocode error: {e}");
                            suggestions.set(vec![]);
                            show_suggestions.set(false);
                        }
                    }
                }
                if geocode_epoch.get() == epoch {
                    is_loading.set(false);
                }
            });
        },
        500.0,
    );

    // On input event: update value and trigger debounced geocode
    let debounced_for_input = debounced_geocode.clone();
    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        input_value.set(val.clone());
        debounced_for_input(val);
    };

    // Helper: select a feature (used by both submit and suggestion click)
    let select_feature = move |feature: &PhotonFeature| {
        let lat = feature.lat();
        let lng = feature.lng();
        let label = feature.display_label();
        input_value.set(label.clone());
        show_suggestions.set(false);
        suggestions.set(vec![]);
        save_address(saved_address, label, lat, lng);
        on_address_selected.run((lat, lng));
    };

    // On form submit (Enter key or Search button): select first suggestion or trigger immediate geocode
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Invalidate any pending debounced geocode
        geocode_epoch.update(|g| *g += 1);
        let current_suggestions = suggestions.get();
        if !current_suggestions.is_empty() && show_suggestions.get() {
            // Select the first suggestion
            select_feature(&current_suggestions[0]);
        } else {
            // Trigger geocoding directly (bypass debounce for immediate feedback)
            let val = input_value.get();
            let epoch = geocode_epoch.get();
            if val.len() >= 3 {
                is_loading.set(true);
                spawn_local(async move {
                    match geocode_address(&val).await {
                        Ok(results) => {
                            if geocode_epoch.get() == epoch {
                                suggestions.set(results);
                                show_suggestions.set(true);
                            }
                        }
                        Err(e) => {
                            if geocode_epoch.get() == epoch {
                                log::error!("[AddressInput] submit geocode error: {e}");
                                suggestions.set(vec![]);
                                show_suggestions.set(false);
                            }
                        }
                    }
                    if geocode_epoch.get() == epoch {
                        is_loading.set(false);
                    }
                });
            }
        }
    };

    // Handle suggestion click
    let on_select_suggestion = move |feature: PhotonFeature| {
        select_feature(&feature);
    };

    // Clear button handler (per D-08)
    let on_clear = move |_: leptos::ev::MouseEvent| {
        input_value.set(String::new());
        suggestions.set(vec![]);
        show_suggestions.set(false);
        clear_address(saved_address);
        on_address_cleared.run(());
    };

    view! {
        <form class="address-input-container" on:submit=on_submit>
            <div style="position:relative;flex:1">
                <input
                    type="text"
                    class="address-input"
                    placeholder=move || t("address_placeholder", lang.get())
                    prop:value=move || input_value.get()
                    on:input=on_input
                />
                // Suggestion dropdown
                {move || {
                    let items = suggestions.get();
                    if show_suggestions.get() && !items.is_empty() {
                        Some(view! {
                            <ul class="address-suggestions">
                                {items
                                    .into_iter()
                                    .map(|feature| {
                                        let label = feature.display_label();
                                        let f = feature.clone();
                                        view! {
                                            <li
                                                class="address-suggestion-item"
                                                on:click=move |_| {
                                                    on_select_suggestion(f.clone());
                                                }
                                            >
                                                {label}
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </ul>
                        })
                    } else {
                        None
                    }
                }}
            </div>
            <button
                type="submit"
                class="address-search-btn"
                disabled=move || travel_loading.get() || is_loading.get()
            >
                {move || {
                    let l = lang.get();
                    if travel_loading.get() {
                        view! { <span class="spinner"></span> {t("calculating", l)} }.into_any()
                    } else if is_loading.get() {
                        view! { <span class="spinner"></span> {t("searching", l)} }.into_any()
                    } else {
                        view! { {t("search", l)} }.into_any()
                    }
                }}
            </button>
            // Clear button: visible when address is active (initial_coords is Some)
            {move || {
                if initial_coords.get().is_some() {
                    Some(view! {
                        <button
                            type="button"
                            class="address-clear-btn"
                            on:click=on_clear
                        >
                            {t("clear_address", lang.get())}
                        </button>
                    })
                } else {
                    None
                }
            }}
        </form>
    }
}
