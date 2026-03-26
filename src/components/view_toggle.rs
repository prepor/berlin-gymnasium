use leptos::prelude::*;

use crate::i18n::{t, use_language};

/// Toggle button to switch between list view and map view.
#[component]
pub fn ViewToggle(
    is_map_view: Signal<bool>,
    on_toggle: Callback<()>,
) -> impl IntoView {
    let lang = use_language();
    let list_class = move || {
        if is_map_view.get() {
            "view-toggle-btn"
        } else {
            "view-toggle-btn active"
        }
    };
    let map_class = move || {
        if is_map_view.get() {
            "view-toggle-btn active"
        } else {
            "view-toggle-btn"
        }
    };

    view! {
        <div class="view-toggle">
            <button class=list_class on:click=move |_| on_toggle.run(())>
                {move || t("list", lang.get())}
            </button>
            <button class=map_class on:click=move |_| on_toggle.run(())>
                {move || t("map", lang.get())}
            </button>
        </div>
    }
}
