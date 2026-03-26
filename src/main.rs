use leptos::prelude::*;

mod app;
mod components;
mod models;
mod pages;
mod services;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(app::App);
}
