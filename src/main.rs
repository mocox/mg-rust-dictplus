
mod constants;
mod components;
mod views;
mod mg_routes;
pub mod library;
pub mod backend;
pub mod models;

use constants::{FAVICON, MAIN_CSS, TAILWIND_CSS};
use dioxus::prelude::*;
use crate::mg_routes::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}


