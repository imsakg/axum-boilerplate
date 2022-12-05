use crate::{crypto, model::User, state::AppState, validation, Error, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/me", axum::routing::get(me))
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

async fn me(crate::identity::Authenticated(user): crate::identity::Authenticated) -> Json<User> {
    Json(user)
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
}

async fn login(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> Result<Json<TokenResponse>> {
    let email = validation::email(&input.email).map_err(|_| Error::LoginFail)?;
    if input.password.len() > 128 {
        return Err(Error::LoginFail);
    }
    state
        .run(move |state| {
            static DUMMY: OnceLock<String> = OnceLock::new();
            let dummy = DUMMY.get_or_init(|| {
                crypto::hash_password("unused timing comparison credential")
                    .expect("Argon2 parameters are valid")
            });
            let user = state.db.user_by_email(&email)?;
            let hash = user
                .as_ref()
                .map(|u| u.password_hash.as_str())
                .unwrap_or(dummy);
            let verified = crypto::verify_password(&input.password, hash);
            let user = user.filter(|_| verified).ok_or(Error::LoginFail)?;
            let access_token = state
                .tokens
                .issue(user.id, user.token_version, crypto::now())?;
            Ok(Json(TokenResponse {
                access_token,
                token_type: "Bearer",
                expires_in: state.config.access_ttl,
            }))
        })
        .await
}
