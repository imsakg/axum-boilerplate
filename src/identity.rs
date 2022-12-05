use crate::{model::User, state::AppState, Error, Result};
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

/// An identity resolved from a signed bearer token and the current database version.
pub struct Authenticated(pub User);

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Authenticated {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        if parts.headers.get_all(AUTHORIZATION).iter().count() != 1 {
            return Err(Error::Unauthorized);
        }
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(Error::Unauthorized)?;
        let (scheme, token) = header.split_once(' ').ok_or(Error::Unauthorized)?;
        if !scheme.eq_ignore_ascii_case("Bearer")
            || token.is_empty()
            || token.len() > 4096
            || token.chars().any(char::is_whitespace)
        {
            return Err(Error::Unauthorized);
        }
        let claims = state.tokens.verify(token)?;
        let id = claims.sub.parse::<i64>().map_err(|_| Error::Unauthorized)?;
        let user = state
            .run(move |state| {
                state
                    .db
                    .user_by_id(id)?
                    .filter(|u| u.token_version == claims.version)
                    .ok_or(Error::Unauthorized)
            })
            .await?;
        Ok(Self(user))
    }
}
