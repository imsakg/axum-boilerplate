use crate::{crypto, model::User, state::AppState, validation, Error, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
        .route("/api/auth/logout", post(logout))
        .route(
            "/api/auth/change-password",
            post(crate::profile::change_password),
        )
        .route(
            "/api/me",
            axum::routing::get(me).patch(crate::profile::update),
        )
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
    refresh_token: String,
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RefreshInput {
    refresh_token: String,
}

fn validate_token(token: &str) -> Result<()> {
    if token.len() != 64 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

async fn logout(
    State(state): State<AppState>,
    Json(input): Json<RefreshInput>,
) -> Result<StatusCode> {
    if validate_token(&input.refresh_token).is_ok() {
        state
            .run(move |state| {
                state
                    .db
                    .logout_session(&crypto::fingerprint(&input.refresh_token), crypto::now())
            })
            .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshInput>,
) -> Result<Json<TokenResponse>> {
    validate_token(&input.refresh_token)?;
    state
        .run(move |state| {
            let refresh_token = crypto::opaque_token();
            let (user_id, _) = state.db.rotate_session(
                &crypto::fingerprint(&input.refresh_token),
                &crypto::fingerprint(&refresh_token),
                crypto::now() + state.config.refresh_ttl,
                crypto::now(),
            )?;
            let user = state.db.user_by_id(user_id)?.ok_or(Error::Unauthorized)?;
            let access_token = state
                .tokens
                .issue(user.id, user.token_version, crypto::now())?;
            Ok(Json(TokenResponse {
                refresh_token,
                access_token,
                token_type: "Bearer",
                expires_in: state.config.access_ttl,
            }))
        })
        .await
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
            let refresh_token = crypto::opaque_token();
            state.db.create_session(
                user.id,
                &crypto::fingerprint(&refresh_token),
                &uuid::Uuid::new_v4().to_string(),
                crypto::now() + state.config.refresh_ttl,
            )?;
            Ok(Json(TokenResponse {
                refresh_token,
                access_token,
                token_type: "Bearer",
                expires_in: state.config.access_ttl,
            }))
        })
        .await
}
