use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::state::AppState;

#[component]
pub fn DetailPage() -> impl IntoView {
    let params = use_params_map();
    let state = use_context::<AppState>().expect("AppState must be provided");

    let school = move || {
        let id = params.read().get("id");
        id.and_then(|id| state.schools.iter().find(|s| s.school_id == id).cloned())
    };

    view! {
        {move || {
            match school() {
                Some(s) => {
                    view! {
                        <main>
                            <a href="/">"Zurueck zur Liste"</a>
                            <h1>{s.name}</h1>
                            <h2>{s.district}</h2>
                        </main>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <main>
                            <a href="/">"Zurueck zur Liste"</a>
                            <h1>"Schule nicht gefunden"</h1>
                        </main>
                    }
                        .into_any()
                }
            }
        }}
    }
}
