use dioxus::prelude::*;
use crate::library::page_title::PageTitle;

#[component]
pub fn RadicalView() -> Element {
    rsx! {
        div {
            class: "p-3 mt-12",
        PageTitle {
            title: "Radicals"
        }
        div {
            class: "p-3",
            "This is testing the radical view"
        }
            }

    }
}