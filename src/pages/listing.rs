use leptos::prelude::*;

use crate::state::AppState;

#[component]
pub fn ListingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");
    let count = state.schools.len();
    let schools = state.schools.clone();

    view! {
        <main>
            <h1>"Berliner Gymnasien"</h1>
            <p>{format!("{} Schulen gefunden", count)}</p>
            <ul>
                {schools
                    .into_iter()
                    .map(|school| {
                        let href = format!("/school/{}", school.school_id);
                        view! {
                            <li>
                                <a href=href>{school.name}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </main>
    }
}
