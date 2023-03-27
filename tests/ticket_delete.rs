mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn delete_removes_the_ticket_and_does_not_reuse_identifiers() {
    let app = app();
    let token = account(&app, "mert@example.com").await;
    let (_, ticket) = request(
        &app,
        "POST",
        "/api/tickets",
        json!({"title":"Disposable"}),
        Some(&token),
    )
    .await;
    let path = format!("/api/tickets/{}", ticket["id"]);
    assert_eq!(
        request(&app, "DELETE", &path, Value::Null, Some(&token))
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        request(&app, "GET", &path, Value::Null, Some(&token))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(&app, "DELETE", &path, Value::Null, Some(&token))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let (_, next) = request(
        &app,
        "POST",
        "/api/tickets",
        json!({"title":"Next"}),
        Some(&token),
    )
    .await;
    assert!(next["id"].as_i64().unwrap() > ticket["id"].as_i64().unwrap());
}
