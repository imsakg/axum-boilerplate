use super::Database;
use crate::{Error, Result};
use rusqlite::{params, OptionalExtension};

impl Database {
    pub fn create_session(
        &self,
        user_id: i64,
        hash: &str,
        family: &str,
        expires: u64,
    ) -> Result<()> {
        self.with(|c| {
            c.execute("INSERT INTO refresh_tokens(hash,user_id,family_id,expires_at) VALUES (?1,?2,?3,?4)",params![hash,user_id,family,expires])?;
            Ok(())
        })
    }
    /// Rotation and consumption are atomic. A replay revokes every token in its family.
    pub fn rotate_session(
        &self,
        old_hash: &str,
        new_hash: &str,
        expires: u64,
        now: u64,
    ) -> Result<(i64, String)> {
        self.with(|c| {
            let tx=c.transaction()?;
            let row=tx.query_row("SELECT user_id,family_id,expires_at,revoked_at FROM refresh_tokens WHERE hash=?1",[old_hash],|r| Ok((r.get::<_,i64>(0)?,r.get::<_,String>(1)?,r.get::<_,u64>(2)?,r.get::<_,Option<i64>>(3)?))).optional()?;
            let (user,family,old_expiry,revoked)=row.ok_or(Error::Unauthorized)?;
            if revoked.is_some() {
                tx.execute("UPDATE refresh_tokens SET revoked_at=?1 WHERE family_id=?2",params![now,family])?;
                tx.commit()?;
                return Err(Error::Unauthorized);
            }
            if old_expiry<=now { return Err(Error::Unauthorized); }
            tx.execute("UPDATE refresh_tokens SET revoked_at=?1 WHERE hash=?2",params![now,old_hash])?;
            tx.execute("INSERT INTO refresh_tokens(hash,user_id,family_id,expires_at) VALUES (?1,?2,?3,?4)",params![new_hash,user,family,expires])?;
            tx.commit()?;
            Ok((user,family))
        })
    }
}
