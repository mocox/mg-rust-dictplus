use serde::{Deserialize, Serialize};

/// Represents a radical entity from the database
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Radical {
    pub index: i32,
    pub simplified: Option<String>,
    pub traditional: String,
    pub pinyin: String,
    pub strokes: i32,
}