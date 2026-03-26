use leptos::prelude::*;

use crate::i18n::{use_language, Language};

#[component]
pub fn LanguageSwitcher() -> impl IntoView {
    let lang = use_language();

    view! {
        <div class="language-switcher">
            <button
                class=move || if lang.get() == Language::De { "lang-btn active" } else { "lang-btn" }
                on:click=move |_| lang.set(Language::De)
            >
                "DE"
            </button>
            <button
                class=move || if lang.get() == Language::En { "lang-btn active" } else { "lang-btn" }
                on:click=move |_| lang.set(Language::En)
            >
                "EN"
            </button>
        </div>
    }
}
