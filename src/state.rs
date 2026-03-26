use leptos::prelude::*;

use crate::models::{load_schools, School};

/// Application-wide state holding the loaded school data.
#[derive(Clone, Debug)]
pub struct AppState {
    pub schools: Vec<School>,
}

/// Load schools and provide AppState via Leptos context.
pub fn provide_app_state() {
    let schools = load_schools();
    provide_context(AppState { schools });
}
