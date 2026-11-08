use dioxus::prelude::*;
use crate::library::page_title::PageTitle;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "mt-12 p-3",
            PageTitle {
                title: "Home!!"
            }
            p{
                "Mo!! Dioxus is a Rust framework for building high-performance web applications."
            }
        }
    }
}