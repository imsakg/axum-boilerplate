use super::Database;
use crate::{model::Ticket, validation, Error, Result};
use rusqlite::{params, OptionalExtension};
impl Database {
    pub fn delete_ticket(&self, owner: i64, id: i64) -> Result<()> {
        self.with(|c| {
            let changed = c.execute(
                "DELETE FROM tickets WHERE id=?1 AND owner_id=?2",
                params![id, owner],
            )?;
            if changed != 1 {
                return Err(Error::NotFound);
            }
            Ok(())
        })
    }
    pub fn update_ticket(
        &self,
        owner: i64,
        id: i64,
        title: Option<&str>,
        status: Option<&str>,
    ) -> Result<Ticket> {
        if title.is_none() && status.is_none() {
            return Err(Error::BadRequest("At least one field is required"));
        }
        let title = title.map(validation::title).transpose()?;
        if status.is_some_and(|s| !matches!(s, "open" | "closed")) {
            return Err(Error::BadRequest("Invalid ticket status"));
        }
        self.with(|c| {
            let changed=c.execute("UPDATE tickets SET title=COALESCE(?1,title),status=COALESCE(?2,status),updated_at=unixepoch() WHERE id=?3 AND owner_id=?4",params![title,status,id,owner])?;
            if changed!=1 { return Err(Error::NotFound); }
            Ok(c.query_row("SELECT * FROM tickets WHERE id=?1 AND owner_id=?2",params![id,owner],Ticket::from_row)?)
        })
    }
    pub fn list_tickets(&self, owner: i64, limit: i64, offset: i64) -> Result<Vec<Ticket>> {
        if !(1..=100).contains(&limit) || !(0..=100000).contains(&offset) {
            return Err(Error::BadRequest("Invalid pagination bounds"));
        }
        self.with(|c| {
            let mut stmt = c.prepare(
                "SELECT * FROM tickets WHERE owner_id=?1 ORDER BY id ASC LIMIT ?2 OFFSET ?3",
            )?;
            let rows = stmt.query_map(params![owner, limit, offset], Ticket::from_row)?;
            Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
        })
    }
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
