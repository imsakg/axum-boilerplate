use super::Database;
use crate::{model::User, Result};
use rusqlite::{params, OptionalExtension};

impl Database {
    pub fn change_password(&self, id: i64, expected_hash: &str, new_hash: &str) -> Result<()> {
        self.with(|c| {
            let tx=c.transaction()?;
            let updated=tx.execute("UPDATE users SET password_hash=?1,token_version=token_version+1 WHERE id=?2 AND password_hash=?3",params![new_hash,id,expected_hash])?;
            if updated!=1 { return Err(crate::Error::Unauthorized); }
            tx.execute("DELETE FROM refresh_tokens WHERE user_id=?1",[id])?;
            tx.execute("DELETE FROM challenges WHERE user_id=?1",[id])?;
            tx.execute("DELETE FROM outbox WHERE user_id=?1",[id])?;
            tx.commit()?;
            Ok(())
        })
    }
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
