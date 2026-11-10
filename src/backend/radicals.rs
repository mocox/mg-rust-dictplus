use dioxus::fullstack::{server, Json, StatusCode};
use dioxus::fullstack::extract::Path;
use dioxus::fullstack::response::{IntoResponse, Response};
use rusqlite::Row;
use crate::backend::data_connection;
use crate::models::radical::Radical;

/// Retrieves all radicals from the radicals table
///
/// # Returns
/// * `Result<Vec<Radical>>` - A Result containing a vector of all radical entries or an error

pub fn get_all_radicals() -> Result<Vec<Radical>, Box<dyn std::error::Error>> {
    let conn = data_connection::connect_to_db()?;

    let mut stmt = conn.prepare("SELECT `Index`, Simplified, Traditional, Pinyin, Strokes FROM AllRadicals ORDER BY `Index`")?;
    let radical_iter = stmt.query_map([], |row| build_rad_from_row(row))?;

    radical_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

/// Retrieves radicals filtered by stroke count
///
/// # Returns
/// * `Result<Vec<Radical>>` - A Result containing a vector of matching radical entries or an error

pub fn get_radicals_by_strokes(strokes: i32) -> Result<Vec<Radical>, Box<dyn std::error::Error>> {
    let conn = data_connection::connect_to_db()?;

    let mut stmt = conn.prepare("SELECT `Index`, Simplified, Traditional, Pinyin, Strokes FROM AllRadicals WHERE Strokes = ? ORDER BY `Index`")?;
    let radical_iter = stmt.query_map([strokes], |row| build_rad_from_row(row))?;

    radical_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn build_rad_from_row(row: &Row) -> Result<Radical, rusqlite::Error> {
    Ok(Radical {
        index: row.get(0)?,
        simplified: row.get(1)?,
        traditional: row.get(2)?,
        pinyin: row.get(3)?,
        strokes: row.get(4)?,
    })
}

/*
pub async fn get_rads() -> Response {
    match get_all_radicals() {
        Ok(radicals) => Json(radicals).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_rads_by_strokes(Path(strokes): Path<i32>) -> Response {
    match get_radicals_by_strokes(strokes) {
        Ok(radicals) => Json(radicals).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
*/
#[cfg(test)]
mod tests {
    use super::{get_all_radicals, get_radicals_by_strokes};

    #[test]
    fn loads_radicals_from_bundled_database() {
        let radicals = get_all_radicals().expect("bundled radicals database should be readable");

        assert!(!radicals.is_empty());
    }

    #[test]
    fn filters_radicals_by_stroke_count() {
        let all_radicals =
            get_all_radicals().expect("bundled radicals database should be readable");
        let stroke_count = all_radicals
            .first()
            .expect("bundled radicals database should contain radicals")
            .strokes;

        let matching_radicals =
            get_radicals_by_strokes(stroke_count).expect("stroke filter query should succeed");

        assert!(!matching_radicals.is_empty());
        assert!(
            matching_radicals
                .iter()
                .all(|radical| radical.strokes == stroke_count)
        );
    }
}
