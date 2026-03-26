use leptos::prelude::*;

/// Toggle button to switch between list view and map view.
#[component]
pub fn ViewToggle(
    is_map_view: Signal<bool>,
    on_toggle: Callback<()>,
) -> impl IntoView {
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
                "Liste"
            </button>
            <button class=map_class on:click=move |_| on_toggle.run(())>
                "Karte"
            </button>
        </div>
    }
}
