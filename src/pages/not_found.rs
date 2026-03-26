use leptos::prelude::*;

use crate::i18n::{t, use_language};

#[component]
pub fn NotFound() -> impl IntoView {
    let lang = use_language();
    view! {
        <main>
            <h1>{move || t("page_not_found", lang.get())}</h1>
            <p>{move || t("page_not_found_desc", lang.get())}</p>
            <a href="#/">{move || t("to_home", lang.get())}</a>
        </main>
    }
}
