//! Account recovery queues secrets in a local outbox; HTTP responses never reveal them.
use crate::{crypto, state::AppState, validation, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/forgot-password", post(forgot_password))
        .route("/api/auth/reset-password", post(reset_password))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResetInput {
    token: String,
    password: String,
}

fn check_token(token: &str) -> Result<()> {
    if token.len() != 64 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(crate::Error::Unauthorized);
    }
    Ok(())
}

async fn reset_password(
    State(state): State<AppState>,
    Json(input): Json<ResetInput>,
) -> Result<StatusCode> {
    check_token(&input.token)?;
    validation::password(&input.password)?;
    state
        .run(move |state| {
            let hash = crypto::hash_password(&input.password)?;
            state
                .db
                .complete_challenge(&input.token, "reset", crypto::now(), Some(&hash))
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmailInput {
    email: String,
}
async fn forgot_password(
    State(state): State<AppState>,
    Json(input): Json<EmailInput>,
) -> Result<(StatusCode, Json<Value>)> {
    let email = validation::email(&input.email)?;
    state
        .run(move |state| {
            if let Some(user) = state.db.user_by_email(&email)? {
                state.db.issue_challenge(
                    user.id,
                    "reset",
                    &crypto::opaque_token(),
                    crypto::now() + 1800,
                )?;
            }
            Ok(())
        })
        .await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(json!({"message":"If the account exists, recovery instructions have been queued"})),
    ))
}
