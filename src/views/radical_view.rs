use dioxus::prelude::*;

#[component]
pub fn RadicalView() -> Element {
    rsx! {
        div {
            class: "mt-12 p-3",
            "Radicals"
        }
    }
}