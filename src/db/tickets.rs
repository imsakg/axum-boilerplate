use super::Database;
use crate::{model::Ticket, validation, Error, Result};
use rusqlite::{params, OptionalExtension};
impl Database {
    pub fn create_ticket(&self, owner: i64, title: &str) -> Result<Ticket> {
        let title = validation::title(title)?;
        self.with(|c| {
            c.execute(
                "INSERT INTO tickets(owner_id,title) VALUES (?1,?2)",
                params![owner, title],
            )?;
            Ok(c.query_row(
                "SELECT * FROM tickets WHERE id=?1",
                [c.last_insert_rowid()],
                Ticket::from_row,
            )?)
        })
    }
    pub fn ticket(&self, owner: i64, id: i64) -> Result<Ticket> {
        self.with(|c| {
            c.query_row(
                "SELECT * FROM tickets WHERE id=?1 AND owner_id=?2",
                params![id, owner],
                Ticket::from_row,
            )
            .optional()?
            .ok_or(Error::NotFound)
        })
    }
}
