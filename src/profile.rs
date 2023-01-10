use crate::{identity::Authenticated, model::User, state::AppState, Error, Result};
use axum::{extract::State, Json};
use serde::Deserialize;
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
