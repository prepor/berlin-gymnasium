use leptos::prelude::*;

use crate::i18n::{t, t_fmt, use_language};

/// Displays the active filter count badge and a "clear all" button.
/// Hidden when no filters are active.
#[component]
pub fn FilterChips(
    active_count: Signal<usize>,
    on_clear_all: Callback<()>,
) -> impl IntoView {
    let lang = use_language();
    let visible = move || active_count.get() > 0;
    let count_text = move || t_fmt("filters_active", lang.get(), &[&active_count.get().to_string()]);

    view! {
        <div class="filter-chips" style:display=move || if visible() { "flex" } else { "none" }>
            <span class="filter-count-badge">{count_text}</span>
            <button class="filter-clear-btn" on:click=move |_| on_clear_all.run(())>
                {move || t("clear_all_filters", lang.get())}
            </button>
        </div>
    }
}
