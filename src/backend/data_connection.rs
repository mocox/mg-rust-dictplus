use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub struct SqliteConnectionManager {
    path: PathBuf,
}

impl SqliteConnectionManager {
    pub fn file(path: PathBuf) -> Self {
        Self { path }
    }
}

pub mod r2d2 {
    use super::SqliteConnectionManager;
    use rusqlite::Connection;
    use std::fmt;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct Error(rusqlite::Error);

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl std::error::Error for Error {}

    impl From<rusqlite::Error> for Error {
        fn from(error: rusqlite::Error) -> Self {
            Self(error)
        }
    }

    pub struct Pool<M> {
        manager: M,
    }

    impl Pool<SqliteConnectionManager> {
        pub fn builder() -> Builder {
            Builder
        }

        pub fn get(&self) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
            Ok(PooledConnection {
                connection: Connection::open(&self.manager.path)?,
                _manager: PhantomData,
            })
        }
    }

    pub struct Builder;

    impl Builder {
        pub fn max_size(self, _max_size: u32) -> Self {
            self
        }

        pub fn build(self, manager: SqliteConnectionManager) -> Result<Pool<SqliteConnectionManager>, Error> {
            Ok(Pool { manager })
        }
    }

    pub struct PooledConnection<M> {
        connection: Connection,
        _manager: PhantomData<M>,
    }

    impl<M> std::ops::Deref for PooledConnection<M> {
        type Target = Connection;

        fn deref(&self) -> &Self::Target {
            &self.connection
        }
    }

    impl<M> std::ops::DerefMut for PooledConnection<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.connection
        }
    }
}

use r2d2::Pool;

static DB_POOL: OnceLock<Pool<SqliteConnectionManager>> = OnceLock::new();

/// Initializes and returns the global database connection pool
///
/// # Returns
/// * `&'static Pool<SqliteConnectionManager>` - Reference to the connection pool
fn get_pool() -> &'static Pool<SqliteConnectionManager> {
    DB_POOL.get_or_init(|| {
        let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/backend/data-sources")
            .join("dictplus.db");
        let manager = SqliteConnectionManager::file(db_path);
        Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create connection pool")
    })
}

/// Gets a database connection from the pool
///
/// # Returns
/// * `Result<Connection>` - A Result containing the database connection or an error
pub fn connect_to_db() -> Result<r2d2::PooledConnection<SqliteConnectionManager>, r2d2::Error> {
    get_pool().get()
}
