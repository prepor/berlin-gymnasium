use leptos::prelude::*;

use crate::i18n::{t, use_language};
use crate::models::SortField;

/// A dropdown control for sorting the school list.
#[component]
pub fn SortControls(
    current_sort: Signal<SortField>,
    on_sort_change: Callback<SortField>,
    #[prop(optional)] has_travel_times: Option<Signal<bool>>,
) -> impl IntoView {
    let lang = use_language();
    let selected_value = move || current_sort.get().to_query().to_string();

    view! {
        <div class="sort-controls">
            <label class="sort-label" for="sort-select">
                {move || t("sort_by", lang.get())}
            </label>
            <select
                id="sort-select"
                class="sort-select"
                on:change=move |ev| {
                    let value = leptos::prelude::event_target_value(&ev);
                    on_sort_change.run(SortField::from_query(&value));
                }
                prop:value=selected_value
            >
                <option value="name">{move || t("name_az", lang.get())}</option>
                <option value="district">{move || t("district", lang.get())}</option>
                <option value="students">{move || t("student_count_sort", lang.get())}</option>
                {move || {
                    let show = has_travel_times.map(|s| s.get()).unwrap_or(false);
                    let l = lang.get();
                    if show {
                        Some(view! {
                            <option value="travel_walk">{t("travel_walk_sort", l)}</option>
                            <option value="travel_bike">{t("travel_bike_sort", l)}</option>
                            <option value="travel_car">{t("travel_car_sort", l)}</option>
                        })
                    } else {
                        None
                    }
                }}
            </select>
        </div>
    }
}
