
use dioxus::prelude::*;
use crate::backend::dictionary_apis::get_dict_items;
use crate::models::dictionary_model::DictionaryModel;

#[component]
pub fn DictionaryView() -> Element {
    let mut draft_query = use_signal(|| String::new());
    let mut search_term = use_signal(|| String::new());
    let results = use_server_future(move || {
        let term = search_term();
        async move {
            if term.trim().is_empty() {
                Ok(Vec::<DictionaryModel>::new())
            } else {
                get_dict_items(term).await
            }
        }
    })?;

    rsx! {
        div {
            div {
                class: "mt-12 p-3",
                h1 { "Dictionary View" }
                p { "Search Chinese entries by meaning." }
            }
            div {
                class: "p-3",
                label {
                    "Search by Meaning: "
                    input {
                        class: "ml-2 text-white",
                        value: "{draft_query}",
                        oninput: move |event| {
                            draft_query.set(event.value().clone());
                        }
                    }
                    button {
                        class: "ml-2 bg-blue-500 text-white px-4 py-2 rounded",
                        onclick: move |_| {
                            search_term.set(draft_query());
                        },
                        "Search"
                    }
                }
            }

            match &*results.read() {
                None => rsx! { p { "Loading dictionary items..." } },
                Some(Err(_)) => rsx! { p { "Unable to load dictionary items. Please try again later." } },
                Some(Ok(items)) if items.is_empty() => rsx! { p { "No dictionary items found." } },
                Some(Ok(items)) => rsx! {
                    table {
                        class: "table-fixed mx-auto border-collapse border border-white w-[800px] mb-8",
                        thead {
                            tr {
                                class: "bg-white text-black",
                                th { class: "border border-gray-300 p-3 text-left w-16", "Index" }
                                th { class: "border border-gray-300 p-3 text-left w-24", "Simplified" }
                                th { class: "border border-gray-300 p-3 text-left w-24", "Traditional" }
                                th { class: "border border-gray-300 p-3 text-left w-48", "Pinyin" }
                                th { class: "border border-gray-300 p-3 text-left w-[200px]", "Meaning" }
                                th { class: "border border-gray-300 p-3 text-left w-24", "Strokes" }
                            }
                        }
                        tbody {
                            for item in items {
                                tr {
                                    td { class: "border border-gray-300 p-3", "{item.id}" }
                                    td { class: "border border-gray-300 p-3", "{item.simplified.as_deref().unwrap_or(\"\")}" }
                                    td { class: "border border-gray-300 p-3", "{item.traditional}" }
                                    td { class: "border border-gray-300 p-3", "{item.true_pinyin}" }
                                    td { class: "border border-gray-300 p-3", "{item.meaning}" }
                                    td { class: "border border-gray-300 p-3", "{item.strokes}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}