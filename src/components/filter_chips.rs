use leptos::prelude::*;

/// Displays the active filter count badge and a "clear all" button.
/// Hidden when no filters are active.
#[component]
pub fn FilterChips(
    active_count: Signal<usize>,
    on_clear_all: Callback<()>,
) -> impl IntoView {
    let visible = move || active_count.get() > 0;
    let count_text = move || format!("{} Filter aktiv", active_count.get());

    view! {
        <div class="filter-chips" style:display=move || if visible() { "flex" } else { "none" }>
            <span class="filter-count-badge">{count_text}</span>
            <button class="filter-clear-btn" on:click=move |_| on_clear_all.run(())>
                "Alle Filter loeschen"
            </button>
        </div>
    }
}
