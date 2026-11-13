use dioxus::core::Element;
use dioxus::core_macro::component;
use dioxus::prelude::*;

#[component]
pub fn DictionaryView() -> Element {
    
    rsx! {
        div {
            class: "mt-12 p-3",
            h1 { "Dictionary View" }
            p { "This is the dictionary view." }
        }
    }
}