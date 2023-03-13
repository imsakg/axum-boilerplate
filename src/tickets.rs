use crate::{identity::Authenticated, model::Ticket, state::AppState, Result};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/tickets", post(create).get(list))
        .route("/api/tickets/:id", axum::routing::get(read))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ListInput {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list(
    State(state): State<AppState>,
    Authenticated(user): Authenticated,
    axum::extract::Query(input): axum::extract::Query<ListInput>,
) -> Result<Json<serde_json::Value>> {
    let limit = input.limit.unwrap_or(20);
    let offset = input.offset.unwrap_or(0);
    let items = state
        .run(move |state| state.db.list_tickets(user.id, limit, offset))
        .await?;
    Ok(Json(
        serde_json::json!({"items":items,"limit":limit,"offset":offset}),
    ))
}

async fn read(
    State(state): State<AppState>,
    Authenticated(user): Authenticated,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<Ticket>> {
    Ok(Json(
        state.run(move |state| state.db.ticket(user.id, id)).await?,
    ))
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
