//! SQLite storage. Run repository operations on a blocking worker, not a reactor.
mod challenges;
mod sessions;
mod users;
use crate::{Error, Result};
use rusqlite::Connection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.pragma_update(None, "foreign_keys", true)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        let db = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        db.migrate()?;
        Ok(db)
    }
    pub fn with<T>(&self, f: impl FnOnce(&mut Connection) -> Result<T>) -> Result<T> {
        let mut connection = self.connection.lock().map_err(|_| Error::Internal)?;
        f(&mut connection)
    }
    pub fn migrate(&self) -> Result<()> {
        self.with(|connection| {
            let version: i64 =
                connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
            if version > 1 {
                return Err(Error::Internal);
            }
            if version == 0 {
                let tx = connection.transaction()?;
                tx.execute_batch(include_str!("../migrations/001_initial.sql"))?;
                tx.commit()?;
            }
            Ok(())
        })
    }
    pub fn schema_version(&self) -> Result<i64> {
        self.with(|c| Ok(c.pragma_query_value(None, "user_version", |r| r.get(0))?))
    }
    pub fn foreign_keys_enabled(&self) -> Result<bool> {
        self.with(|c| Ok(c.pragma_query_value(None, "foreign_keys", |r| r.get(0))?))
    }
    pub fn ready(&self) -> Result<()> {
        self.with(|c| {
            c.query_row("SELECT 1 FROM users LIMIT 1", [], |_| Ok(()))
                .optional()?;
            Ok(())
        })
    }
}
use rusqlite::OptionalExtension;

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::SqliteFailure(code, _)
                if code.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE =>
            {
                Self::Conflict
            }
            _ => Self::Internal,
        }
    }
}
