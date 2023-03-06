mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn read_returns_only_an_owned_ticket() {
    let app = app();
    let token = account(&app, "mert@example.com").await;
    let (_, ticket) = request(
        &app,
        "POST",
        "/api/tickets",
        json!({"title":"Ship API"}),
        Some(&token),
    )
    .await;
    let path = format!("/api/tickets/{}", ticket["id"]);
    assert_eq!(
        request(&app, "GET", &path, Value::Null, Some(&token)).await,
        (StatusCode::OK, ticket)
    );
    assert_eq!(
        request(&app, "GET", "/api/tickets/99999", Value::Null, Some(&token))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
}
