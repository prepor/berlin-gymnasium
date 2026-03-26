use leptos::prelude::*;

use crate::i18n::{t, use_language};

/// Reusable section wrapper for the detail page.
/// Renders a heading and either children or "No data" when empty.
#[component]
pub fn DetailSection(
    title: &'static str,
    children: Children,
    #[prop(optional)] empty: bool,
) -> impl IntoView {
    let lang = use_language();
    view! {
        <section class="detail-section">
            <h2>{title}</h2>
            {if empty {
                view! { <p class="keine-angabe">{move || t("no_data", lang.get())}</p> }.into_any()
            } else {
                children().into_any()
            }}
        </section>
    }
}
