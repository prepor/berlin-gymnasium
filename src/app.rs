use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages::detail::DetailPage;
use crate::pages::listing::ListingPage;
use crate::pages::not_found::NotFound;
use crate::state::provide_app_state;

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=ListingPage />
                <Route path=path!("/school/:id") view=DetailPage />
            </Routes>
        </Router>
    }
}
