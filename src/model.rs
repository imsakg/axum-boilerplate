//! Persisted domain records. Credentials never serialize into API responses.
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub display_name: String,
    pub verified: bool,
    pub created_at: i64,
    #[serde(skip)]
    pub(crate) password_hash: String,
    #[serde(skip)]
    pub(crate) token_version: i64,
}

#[derive(Clone, Serialize)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}
impl Ticket {
    pub(crate) fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            title: row.get("title")?,
            status: row.get("status")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

impl User {
    pub(crate) fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            email: row.get("email")?,
            display_name: row.get("display_name")?,
            verified: row.get("verified")?,
            created_at: row.get("created_at")?,
            password_hash: row.get("password_hash")?,
            token_version: row.get("token_version")?,
        })
    }
}
