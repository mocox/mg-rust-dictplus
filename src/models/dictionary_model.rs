use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictionaryModel {
    pub index: i32,
    pub simplified: Option<String>,
    pub traditional: String,
    pub true_pinyin: String,
    pub meaning: String,
    pub strokes: i32,
}