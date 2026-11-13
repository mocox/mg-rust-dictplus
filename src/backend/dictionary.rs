use rusqlite::{Error, Row};
use crate::backend::data_connection;
use crate::models::dictionary_model::DictionaryModel;

pub fn get_entries(criteria: String) -> Result<Vec<DictionaryModel>, Box<dyn std::error::Error>> {
    let conn = data_connection::connect_to_db()?;

    let mut stmt = conn.prepare("SELECT `Id`, Simplified, Traditional, TruePinyin, Meaning, TotalStrokes FROM DictionaryZH WHERE Meaning LIKE '%' || ? || '%' ORDER BY `Id`")?;
    let dict_iter = stmt.query_map([criteria], |row| build_dict_from_row(row))?;

    dict_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn build_dict_from_row(row: &Row) -> Result<DictionaryModel, Error> {
    Ok(DictionaryModel {
        id: row.get(0)?,
        simplified: row.get(1)?,
        traditional: row.get(2)?,
        true_pinyin: row.get(3)?,
        meaning: row.get(4)?,
        strokes: row.get(5)?,
    })
}