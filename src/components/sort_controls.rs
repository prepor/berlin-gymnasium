use leptos::prelude::*;

use crate::models::SortField;

/// A dropdown control for sorting the school list.
#[component]
pub fn SortControls(
    current_sort: Signal<SortField>,
    on_sort_change: Callback<SortField>,
) -> impl IntoView {
    let selected_value = move || current_sort.get().to_query().to_string();

    view! {
        <div class="sort-controls">
            <label class="sort-label" for="sort-select">
                "Sortieren:"
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
                <option value="name">"Name (A-Z)"</option>
                <option value="district">"Bezirk"</option>
                <option value="students">"Schueleranzahl"</option>
            </select>
        </div>
    }
}
