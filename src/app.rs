use leptos::prelude::*;

use crate::hash_router::{provide_hash_router, HashLocation};
use crate::pages::detail::DetailPage;
use crate::pages::listing::ListingPage;
use crate::pages::not_found::NotFound;
use crate::state::provide_app_state;

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();
    provide_hash_router();

    let location = use_context::<HashLocation>().expect("HashLocation must be provided");

    view! {
        {move || {
            let path = location.path.get();
            if path == "/" || path.is_empty() {
                view! { <ListingPage /> }.into_any()
            } else if let Some(id) = path.strip_prefix("/school/") {
                let id = id.to_string();
                view! { <DetailPage id=id /> }.into_any()
            } else {
                view! { <NotFound /> }.into_any()
            }
        }}
    }
}
