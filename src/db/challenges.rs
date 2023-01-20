use super::Database;
use crate::{crypto, Error, Result};
use rusqlite::{params, OptionalExtension};

impl Database {
    /// The challenge table holds fingerprints; raw tokens live only in the trusted outbox.
    pub fn issue_challenge(&self, user: i64, kind: &str, token: &str, expires: u64) -> Result<()> {
        if !matches!(kind, "verify" | "reset") {
            return Err(Error::BadRequest("Invalid challenge kind"));
        }
        self.with(|c| {
            let tx = c.transaction()?;
            tx.execute(
                "DELETE FROM challenges WHERE user_id=?1 AND kind=?2",
                params![user, kind],
            )?;
            tx.execute(
                "DELETE FROM outbox WHERE user_id=?1 AND kind=?2",
                params![user, kind],
            )?;
            tx.execute(
                "INSERT INTO challenges(hash,user_id,kind,expires_at) VALUES (?1,?2,?3,?4)",
                params![crypto::fingerprint(token), user, kind, expires],
            )?;
            tx.execute(
                "INSERT INTO outbox(user_id,kind,token,expires_at) VALUES (?1,?2,?3,?4)",
                params![user, kind, token, expires],
            )?;
            tx.commit()?;
            Ok(())
        })
    }
    /// Consume and apply the action in the same transaction, including session revocation.
    pub fn complete_challenge(
        &self,
        token: &str,
        kind: &str,
        now: u64,
        password_hash: Option<&str>,
    ) -> Result<()> {
        self.with(|c| {
            let tx=c.transaction()?;
            let user: i64=tx.query_row("SELECT user_id FROM challenges WHERE hash=?1 AND kind=?2 AND expires_at>?3",params![crypto::fingerprint(token),kind,now],|r| r.get(0)).optional()?.ok_or(Error::Unauthorized)?;
            match kind {
                "verify" => {
                    tx.execute("UPDATE users SET verified=1 WHERE id=?1",[user])?;
                    tx.execute("DELETE FROM challenges WHERE user_id=?1 AND kind='verify'",[user])?;
                    tx.execute("DELETE FROM outbox WHERE user_id=?1 AND kind='verify'",[user])?;
                }
                "reset" => {
                    let hash=password_hash.ok_or(Error::BadRequest("A new password hash is required"))?;
                    tx.execute("UPDATE users SET password_hash=?1,token_version=token_version+1 WHERE id=?2",params![hash,user])?;
                    tx.execute("DELETE FROM refresh_tokens WHERE user_id=?1",[user])?;
                    tx.execute("DELETE FROM challenges WHERE user_id=?1",[user])?;
                    tx.execute("DELETE FROM outbox WHERE user_id=?1",[user])?;
                }
                _ => return Err(Error::Unauthorized),
            }
            tx.commit()?; Ok(())
        })
    }
}
