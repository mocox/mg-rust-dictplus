use dioxus::fullstack::{get, ServerFnError};
use crate::models::dictionary_model::DictionaryModel;

fn db_error(error: impl std::fmt::Display) -> ServerFnError {
    eprintln!("Failed to load dictionary items: {error}");
    ServerFnError::ServerError {
        message: "Failed to load dictionary items from the database".to_string(),
        code: 500,
        details: None,
    }
}

#[get("/api/dictionary/by-meaning/:criteria")]
pub async fn get_items(criteria: &String) -> dioxus::Result<Vec<DictionaryModel>, ServerFnError> {
    crate::backend::dictionary::get_entries(criteria).map_err(db_error)
}