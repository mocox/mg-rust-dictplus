use dioxus::prelude::*;


#[component]
pub fn PageTitle(title: String) -> Element {
    rsx! {
        h1 {
            class:"text-center italic",
            "{title}"
            }
    }
}