use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main>
            <h1>"Seite nicht gefunden"</h1>
            <p>"Die angeforderte Seite existiert nicht."</p>
            <a href="/">"Zur Startseite"</a>
        </main>
    }
}
