use crate::{identity::Authenticated, model::Ticket, state::AppState, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/tickets", post(create))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateInput {
    title: String,
}
async fn create(
    State(state): State<AppState>,
    Authenticated(user): Authenticated,
    Json(input): Json<CreateInput>,
) -> Result<(StatusCode, Json<Ticket>)> {
    let ticket = state
        .run(move |state| state.db.create_ticket(user.id, &input.title))
        .await?;
    Ok((StatusCode::CREATED, Json(ticket)))
}
