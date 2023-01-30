//! Account recovery queues secrets in a local outbox; HTTP responses never reveal them.
use crate::{crypto, state::AppState, validation, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/forgot-password", post(forgot_password))
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
