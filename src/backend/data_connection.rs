use std::path::Path;

/// Gets a database connection
///
/// # Returns
/// * `Result<Connection>` - A Result containing a database connection or an error
pub fn connect_to_db() -> Result<rusqlite::Connection, rusqlite::Error> {
    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/backend/data-sources/dictplus.db");
    rusqlite::Connection::open(db_path)
}
