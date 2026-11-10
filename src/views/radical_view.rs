use dioxus::prelude::*;
use crate::backend::radical_apis::get_radicals;
use crate::library::page_title::PageTitle;
use crate::models::radical::Radical;



#[component]
pub fn RadicalView() -> Element {

    let mut selected_strokes = use_signal(|| 1_i32);
    let radicals = use_server_future(get_radicals)?;

    rsx! {
        div {
            class: "p-3 mt-12",
            PageTitle { title: "Radicals" }
            div {
                class: "p-3",
                div {
                    class: "text-center mb-2",
                    label {
                        "Radicals Strokes"
                        select {
                            class: "ml-3",
                            value: "{selected_strokes}",
                            onchange: move |event| {
                                if let Ok(strokes) = event.value().parse::<i32>() {
                                    selected_strokes.set(strokes);
                                }
                            },
                            for strokes in 1..=17 {
                                option { value: "{strokes}", "{strokes}" }
                            }
                        }
                    }

                }
                match &*radicals.read() {
                    None => rsx! { p { "Loading radicals..." } },
                    Some(Err(_)) => rsx! { p { "Unable to load radicals. Please try again later." } },
                    Some(Ok(radicals)) if radicals_for_strokes(radicals, selected_strokes()).next().is_none() => rsx! { p { "No radicals found." } },
                    Some(Ok(radicals)) => rsx! {
                        table {
                            class: "table-fixed mx-auto border-collapse border border-white w-[500px]",
                            thead {
                                tr {
                                    class: "bg-white text-black",
                                    th { class: "border border-gray-300 p-3 text-left",
                                        span {"Traditional" }
                                        }
                                    th { class: "border border-gray-300 p-3 text-left",
                                            span{"Simplified" }
                                        }
                                    th { class: "border border-gray-300 p-3 text-left",
                                        span {"Pinyin" }
                                        }
                                    th { class: "border border-gray-300 p-3 text-left",
                                        span {"Strokes" }
                                        }
                                }
                            }
                            tbody {
                                for radical in radicals_for_strokes(radicals, selected_strokes()) {
                                    tr {
                                        key: "{radical.index}",
                                        td { class: "p-3", "{radical.traditional}" }
                                        td { class: "p-3", "{radical.simplified.as_deref().unwrap_or(\"\")}" }
                                        td { class: "p-3", "{radical.pinyin}" }
                                        td { class: "p-3", "{radical.strokes}" }
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

fn radicals_for_strokes(radicals: &[Radical], strokes: i32) -> impl Iterator<Item = &Radical> {
    radicals.iter().filter(move |radical| radical.strokes == strokes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_radicals_for_each_selection() {
        let radicals = vec![
            Radical { index: 1, simplified: None, traditional: "一".into(), pinyin: "yi".into(), strokes: 1 },
            Radical { index: 2, simplified: None, traditional: "二".into(), pinyin: "er".into(), strokes: 2 },
        ];

        assert_eq!(radicals_for_strokes(&radicals, 1).map(|r| r.index).collect::<Vec<_>>(), vec![1]);
        assert_eq!(radicals_for_strokes(&radicals, 2).map(|r| r.index).collect::<Vec<_>>(), vec![2]);
        assert_eq!(radicals_for_strokes(&radicals, 17).count(), 0);
    }
}

