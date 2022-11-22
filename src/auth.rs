use crate::{crypto, model::User, state::AppState, validation, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/register", post(register))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Credentials {
    email: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> Result<(StatusCode, Json<User>)> {
    let email = validation::email(&input.email)?;
    validation::password(&input.password)?;
    let user = state
        .run(move |state| {
            let hash = crypto::hash_password(&input.password)?;
            state.db.create_user(&email, &hash)
        })
        .await?;
    Ok((StatusCode::CREATED, Json(user)))
}
