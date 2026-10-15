use crate::{identity::Authenticated, model::User, state::AppState, Error, Result};
use axum::{extract::State, Json};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PasswordInput {
    current_password: String,
    new_password: String,
}

pub(crate) async fn change_password(
    State(state): State<AppState>,
    Authenticated(user): Authenticated,
    Json(input): Json<PasswordInput>,
) -> Result<axum::http::StatusCode> {
    crate::validation::password(&input.new_password)?;
    if input.current_password.len() > 128 {
        return Err(Error::Unauthorized);
    }
    state
        .run(move |state| {
            if !crate::crypto::verify_password(&input.current_password, &user.password_hash) {
                return Err(Error::Unauthorized);
            }
            let hash = crate::crypto::hash_password(&input.new_password)?;
            state
                .db
                .change_password(user.id, &user.password_hash, &hash)
        })
        .await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProfileInput {
    display_name: String,
}
pub(crate) async fn update(
    State(state): State<AppState>,
    Authenticated(user): Authenticated,
    Json(input): Json<ProfileInput>,
) -> Result<Json<User>> {
    let name = input.display_name.trim().to_owned();
    if name.chars().count() > 80 || name.chars().any(char::is_control) {
        return Err(Error::BadRequest(
            "Display name must be at most 80 non-control characters",
        ));
    }
    Ok(Json(
        state
            .run(move |state| state.db.update_profile(user.id, &name))
            .await?,
    ))
}
