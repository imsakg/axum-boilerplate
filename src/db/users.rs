use super::Database;
use crate::{model::User, Result};
use rusqlite::{params, OptionalExtension};

impl Database {
    pub fn update_profile(&self, id: i64, name: &str) -> Result<User> {
        self.with(|c| {
            c.execute(
                "UPDATE users SET display_name=?1 WHERE id=?2",
                params![name, id],
            )?;
            Ok(c.query_row("SELECT * FROM users WHERE id=?1", [id], User::from_row)?)
        })
    }
    pub fn create_user(&self, email: &str, password_hash: &str) -> Result<User> {
        self.with(|c| {
            c.execute(
                "INSERT INTO users(email,password_hash) VALUES (?1,?2)",
                params![email, password_hash],
            )?;
            Ok(c.query_row(
                "SELECT * FROM users WHERE id=?1",
                [c.last_insert_rowid()],
                User::from_row,
            )?)
        })
    }
    pub fn user_by_id(&self, id: i64) -> Result<Option<User>> {
        self.with(|c| {
            Ok(
                c.query_row("SELECT * FROM users WHERE id=?1", [id], User::from_row)
                    .optional()?,
            )
        })
    }
    pub fn user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.with(|c| {
            Ok(c.query_row(
                "SELECT * FROM users WHERE email=?1",
                [email],
                User::from_row,
            )
            .optional()?)
        })
    }
}
