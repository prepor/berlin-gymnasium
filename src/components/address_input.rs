use leptos::prelude::*;
use leptos_use::use_debounce_fn_with_arg;
use wasm_bindgen_futures::spawn_local;

use crate::services::geocoding::{geocode_address, PhotonFeature};

/// Address input component with debounced geocoding and suggestion dropdown.
/// Lets the user type an address, see up to 5 suggestions, and select one.
#[component]
pub fn AddressInput(
    on_address_selected: Callback<(f64, f64)>,
    on_address_cleared: Callback<()>,
    initial_coords: Signal<Option<(f64, f64)>>,
    travel_loading: Signal<bool>,
) -> impl IntoView {
    let input_value = RwSignal::new(String::new());
    let suggestions = RwSignal::new(Vec::<PhotonFeature>::new());
    let show_suggestions = RwSignal::new(false);
    let is_loading = RwSignal::new(false);

    // Debounced geocoding: fires 500ms after user stops typing (per D-19)
    let debounced_geocode = use_debounce_fn_with_arg(
        move |query: String| {
            if query.len() < 3 {
                suggestions.set(vec![]);
                show_suggestions.set(false);
                return;
            }
            is_loading.set(true);
            spawn_local(async move {
                match geocode_address(&query).await {
                    Ok(results) => {
                        suggestions.set(results);
                        show_suggestions.set(true);
                    }
                    Err(e) => {
                        log::error!("[AddressInput] geocode error: {e}");
                        suggestions.set(vec![]);
                        show_suggestions.set(false);
                    }
                }
                is_loading.set(false);
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

    // On form submit (Enter key or Suchen button): select first suggestion or trigger immediate geocode
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let current_suggestions = suggestions.get();
        if !current_suggestions.is_empty() && show_suggestions.get() {
            // Select the first suggestion
            let first = &current_suggestions[0];
            let lat = first.lat();
            let lng = first.lng();
            input_value.set(first.display_label());
            show_suggestions.set(false);
            on_address_selected.run((lat, lng));
        } else {
            // Trigger geocoding directly (bypass debounce for immediate feedback)
            let val = input_value.get();
            if val.len() >= 3 {
                is_loading.set(true);
                spawn_local(async move {
                    match geocode_address(&val).await {
                        Ok(results) => {
                            suggestions.set(results);
                            show_suggestions.set(true);
                        }
                        Err(e) => {
                            log::error!("[AddressInput] submit geocode error: {e}");
                            suggestions.set(vec![]);
                            show_suggestions.set(false);
                        }
                    }
                    is_loading.set(false);
                });
            }
        }
    };

    // Handle suggestion click
    let on_select_suggestion = move |feature: PhotonFeature| {
        let lat = feature.lat();
        let lng = feature.lng();
        input_value.set(feature.display_label());
        show_suggestions.set(false);
        suggestions.set(vec![]);
        on_address_selected.run((lat, lng));
    };

    // Clear button handler (per D-08)
    let on_clear = move |_: leptos::ev::MouseEvent| {
        input_value.set(String::new());
        suggestions.set(vec![]);
        show_suggestions.set(false);
        on_address_cleared.run(());
    };

    view! {
        <form class="address-input-container" on:submit=on_submit>
            <div style="position:relative;flex:1">
                <input
                    type="text"
                    class="address-input"
                    placeholder="Adresse eingeben"
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
                    if travel_loading.get() {
                        view! { <span class="spinner"></span> " Berechne..." }.into_any()
                    } else if is_loading.get() {
                        view! { <span class="spinner"></span> " Suche..." }.into_any()
                    } else {
                        view! { "Suchen" }.into_any()
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
                            "Adresse loeschen"
                        </button>
                    })
                } else {
                    None
                }
            }}
        </form>
    }
}
