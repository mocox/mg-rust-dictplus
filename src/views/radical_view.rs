use dioxus::prelude::*;
use crate::backend::radical_apis::get_radicals;
use crate::library::page_title::PageTitle;



#[component]
pub fn RadicalView() -> Element {
    let radicals = use_server_future(get_radicals)?;

    rsx! {
        div {
            class: "p-3 mt-12",
            PageTitle { title: "Radicals" }
            div {
                class: "p-3",
                match &*radicals.read() {
                    None => rsx! { p { "Loading radicals..." } },
                    Some(Err(_)) => rsx! { p { "Unable to load radicals. Please try again later." } },
                    Some(Ok(radicals)) if radicals.is_empty() => rsx! { p { "No radicals found." } },
                    Some(Ok(radicals)) => rsx! {
                        table {
                            class: "table-auto mx-auto",
                            thead {
                                tr {
                                    th { class: "p-3", "Traditional"}
                                    th { class:"p-3", "Simplified" }
                                    th { class: "p-3","Pinyin" }
                                    th { class: "p-3","Strokes" }
                                }
                            }
                            tbody {
                                for radical in radicals {
                                    tr {
                                        key: "{radical.index}",
                                        td { class: "p-3", "{radical.traditional}" }
                                        td { class: "p-3","{radical.simplified.as_deref().unwrap_or(\"\")}" }
                                        td { class: "p-3","{radical.pinyin}" }
                                        td { class: "p-3","{radical.strokes}" }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

