use leptos::prelude::*;

/// Reusable section wrapper for the detail page.
/// Renders a heading and either children or "Keine Angabe" when empty.
#[component]
pub fn DetailSection(
    title: &'static str,
    children: Children,
    #[prop(optional)] empty: bool,
) -> impl IntoView {
    view! {
        <section class="detail-section">
            <h2>{title}</h2>
            {if empty {
                view! { <p class="keine-angabe">"Keine Angabe"</p> }.into_any()
            } else {
                children().into_any()
            }}
        </section>
    }
}
