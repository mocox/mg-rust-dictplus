use dioxus::prelude::*;
use crate::constants::HEADER_SVG;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center mt-12",
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }

            p {
                "Mo!! Dioxus is a Rust framework for building high-performance web applications."
            }
        }
    }
}