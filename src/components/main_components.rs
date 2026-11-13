use dioxus::prelude::*;
use crate::mg_routes::Route;


#[component]
pub fn MainHeader() -> Element {
    rsx! {
        div {
            class: "header fixed top-0 w-full bg-white text-black p-3 h-12",
            div {
                class: "flex flex-row justify-start gap-3",
                Link { to: Route::Home,
                    span { "🔥 Hero! " }
                },
                    Link { to: Route::RadicalView,
                    span { "🈷️ Radicals! " }
                },
                Link { to: Route::DictionaryView,
                    span { "🈷️ Dictionary! " }
                }
            }
        }
    }
}

#[component]
pub fn MainFooter() -> Element {
    rsx! {
        div {
            class: "footer fixed bottom-0 w-full bg-white text-black p-3 h-12",
            "Mo footer Rusty"
        }
    }
}