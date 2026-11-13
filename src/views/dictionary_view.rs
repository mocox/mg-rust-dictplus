
use dioxus::prelude::*;
use crate::backend::dictionary_apis::get_dict_items;
use crate::models::dictionary_model::DictionaryModel;

#[component]
pub fn DictionaryView() -> Element {
    let mut draft_query = use_signal(|| String::new());
    let mut search_term = use_signal(|| String::new());
    let mut cached_items = use_signal(Vec::<DictionaryModel>::new);

    let results = use_resource(move || {
        let term = search_term();
        async move {
            if term.trim().is_empty() {
                Ok(Vec::<DictionaryModel>::new())
            } else {
                get_dict_items(term).await
            }
        }
    });

    // Update the cache whenever a successful result arrives.
    // While the next fetch is in-flight (results == None) we keep showing the
    // previous cached rows, eliminating the "Loading…" flicker.
    use_effect(move || {
        if let Some(Ok(items)) = &*results.read() {
            cached_items.set(items.clone());
        }
    });

    let is_loading = results.read().is_none();
    let has_error = matches!(&*results.read(), Some(Err(_)));
    let is_empty = cached_items.read().is_empty();

    rsx! {
        div {
            div {
                class: "mt-12 p-3 text-center",
                h1 { "Dictionary View" }
                p { "Search Chinese entries by meaning." }
            }
            div {
                class: "p-3 mx-auto w-[800px] flex items-center",
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
                            draft_query.set(String::new());
                        },
                        "Search"
                    }
                }
            }

            if has_error {
                p { "Unable to load dictionary items. Please try again later." }
            } else if is_empty && is_loading {
                p { "Loading dictionary items..." }
            } else if is_empty {
                p { "No dictionary items found." }
            } else {
                table {
                    class: "table-fixed mx-auto border-collapse border border-white w-[800px] mb-8",
                    thead {
                        tr {
                            class: "bg-white text-black",
                            th { class: "border border-gray-300 p-3 text-left w-16", "Index" }
                            th { class: "border border-gray-300 p-3 text-left w-24", "Simplified" }
                            th { class: "border border-gray-300 p-3 text-left w-28", "Traditional" }
                            th { class: "border border-gray-300 p-3 text-left w-48", "Pinyin" }
                            th { class: "border border-gray-300 p-3 text-left w-[200px]", "Meaning" }
                            th { class: "border border-gray-300 p-3 text-left w-24", "Strokes" }
                        }
                    }
                    tbody {
                        for item in cached_items.read().iter() {
                            tr {
                                td { class: "border border-gray-300 p-3", "{item.id}" }
                                td { class: "border border-gray-300 p-3", "{item.simplified.as_deref().unwrap_or(\"\")}" }
                                td { class: "border border-gray-300 p-3 wrap-anywhere", "{item.traditional}" }
                                td { class: "border border-gray-300 p-3", "{item.true_pinyin}" }
                                td { class: "border border-gray-300 p-3 wrap-anywhere", "{item.meaning}" }
                                td { class: "border border-gray-300 p-3", "{item.strokes}" }
                            }
                        }
                    }
                }
            }
        }
    }
}