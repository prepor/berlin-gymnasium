use leptos::prelude::*;

/// Multi-select checkbox group for filtering.
#[component]
fn CheckboxGroup(
    label: &'static str,
    options: Vec<String>,
    selected: Signal<Vec<String>>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    view! {
        <fieldset class="filter-group">
            <legend class="filter-group-label">{label}</legend>
            <div class="filter-options">
                {options
                    .into_iter()
                    .map(|opt| {
                        let opt_for_check = opt.clone();
                        let opt_for_cb = opt.clone();
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
                                {opt.clone()}
                            </label>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </fieldset>
    }
}

/// Tri-state radio group (Alle / Ja / Nein) for boolean filters.
#[component]
fn TriStateRadio(
    label: &'static str,
    name: &'static str,
    selected: Signal<Option<bool>>,
    on_change: Callback<Option<bool>>,
) -> impl IntoView {
    let is_all = move || selected.get().is_none();
    let is_yes = move || selected.get() == Some(true);
    let is_no = move || selected.get() == Some(false);

    view! {
        <fieldset class="filter-group">
            <legend class="filter-group-label">{label}</legend>
            <div class="filter-radios">
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_all
                        on:change=move |_| on_change.run(None)
                    />
                    "Alle"
                </label>
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_yes
                        on:change=move |_| on_change.run(Some(true))
                    />
                    "Ja"
                </label>
                <label class="filter-radio">
                    <input
                        type="radio"
                        name=name
                        prop:checked=is_no
                        on:change=move |_| on_change.run(Some(false))
                    />
                    "Nein"
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
    view! {
        <details class="filter-panel-mobile" open=true>
            <summary class="filter-toggle">"Filter anzeigen / verbergen"</summary>
            <div class="filter-panel">
                <h2 class="filter-heading">"Filter"</h2>

                <CheckboxGroup
                    label="Bezirk"
                    options=districts
                    selected=selected_districts
                    on_toggle=on_toggle_district
                />

                <CheckboxGroup
                    label="Profil"
                    options=profiles
                    selected=selected_profiles
                    on_toggle=on_toggle_profile
                />

                <TriStateRadio
                    label="Grundstaendig (ab Klasse 5)"
                    name="grundstaendig"
                    selected=selected_grundstaendig
                    on_change=on_set_grundstaendig
                />

                <CheckboxGroup
                    label="Fremdsprache"
                    options=languages
                    selected=selected_languages
                    on_toggle=on_toggle_language
                />

                <TriStateRadio
                    label="Ganztag"
                    name="ganztag"
                    selected=selected_ganztag
                    on_change=on_set_ganztag
                />
            </div>
        </details>
    }
}
