use leptos::prelude::*;

use crate::i18n::{profile_label, t, use_language};

/// Multi-select checkbox group for filtering.
#[component]
fn CheckboxGroup(
    label_key: &'static str,
    options: Vec<String>,
    selected: Signal<Vec<String>>,
    on_toggle: Callback<String>,
    #[prop(optional)] display_fn: Option<Callback<String, String>>,
) -> impl IntoView {
    let lang = use_language();
    view! {
        <fieldset class="filter-group">
            <legend class="filter-group-label">{move || t(label_key, lang.get())}</legend>
            <div class="filter-options">
                {options
                    .into_iter()
                    .map(|opt| {
                        let opt_for_check = opt.clone();
                        let opt_for_cb = opt.clone();
                        let label = match &display_fn {
                            Some(f) => f.run(opt.clone()),
                            None => opt.clone(),
                        };
                        let is_checked = move || {
                            selected.get().contains(&opt_for_check)
                        };
                        view! {
                            <label class="filter-checkbox">
                                <input
                                    type="checkbox"
                                    prop:checked=is_checked
                                    on:change=move |_| {
                                        on_toggle.run(opt_for_cb.clone());
                                    }
                                />
                                {label}
                            </label>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </fieldset>
    }
}

/// Tri-state radio group (All / Yes / No) for boolean filters.
#[component]
fn TriStateRadio(
    label_key: &'static str,
    name: &'static str,
    selected: Signal<Option<bool>>,
    on_change: Callback<Option<bool>>,
) -> impl IntoView {
    let lang = use_language();
    let is_all = move || selected.get().is_none();
    let is_yes = move || selected.get() == Some(true);
    let is_no = move || selected.get() == Some(false);

    view! {
        <fieldset class="filter-group">
            <legend class="filter-group-label">{move || t(label_key, lang.get())}</legend>
            <div class="filter-radios">
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_all
                        on:change=move |_| on_change.run(None)
                    />
                    {move || t("all", lang.get())}
                </label>
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_yes
                        on:change=move |_| on_change.run(Some(true))
                    />
                    {move || t("yes", lang.get())}
                </label>
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_no
                        on:change=move |_| on_change.run(Some(false))
                    />
                    {move || t("no", lang.get())}
                </label>
            </div>
        </fieldset>
    }
}

/// The full filter panel with all five filter types.
/// On mobile, the panel is rendered inside a <details> element for collapse/expand.
#[component]
pub fn FilterPanel(
    districts: Vec<String>,
    profiles: Vec<String>,
    languages: Vec<String>,
    selected_districts: Signal<Vec<String>>,
    selected_profiles: Signal<Vec<String>>,
    selected_grundstaendig: Signal<Option<bool>>,
    selected_languages: Signal<Vec<String>>,
    selected_ganztag: Signal<Option<bool>>,
    on_toggle_district: Callback<String>,
    on_toggle_profile: Callback<String>,
    on_toggle_language: Callback<String>,
    on_set_grundstaendig: Callback<Option<bool>>,
    on_set_ganztag: Callback<Option<bool>>,
) -> impl IntoView {
    let lang = use_language();
    view! {
        <details class="filter-panel-mobile" open=true>
            <summary class="filter-toggle">{move || t("show_hide_filters", lang.get())}</summary>
            <div class="filter-panel">
                <h2 class="filter-heading">{move || t("filters", lang.get())}</h2>

                <CheckboxGroup
                    label_key="district"
                    options=districts
                    selected=selected_districts
                    on_toggle=on_toggle_district
                />

                <CheckboxGroup
                    label_key="profile"
                    options=profiles
                    selected=selected_profiles
                    on_toggle=on_toggle_profile
                    display_fn=Callback::new(move |opt: String| {
                        let label = profile_label(&opt, lang.get()).to_string();
                        if label.is_empty() { opt } else { label }
                    })
                />

                <TriStateRadio
                    label_key="grundstaendig"
                    name="grundstaendig"
                    selected=selected_grundstaendig
                    on_change=on_set_grundstaendig
                />

                <CheckboxGroup
                    label_key="foreign_language"
                    options=languages
                    selected=selected_languages
                    on_toggle=on_toggle_language
                />

                <TriStateRadio
                    label_key="ganztag"
                    name="ganztag"
                    selected=selected_ganztag
                    on_change=on_set_ganztag
                />
            </div>
        </details>
    }
}
