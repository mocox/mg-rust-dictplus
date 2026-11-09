use dioxus::prelude::*;

use crate::components::main_components::{MainFooter, MainHeader};
use crate::mg_routes::Route;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            MainHeader {}
            div {
                class: "my-12",
                Outlet::<Route> {}
            }
            MainFooter {}
        }
    }
}