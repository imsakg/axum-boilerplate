mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn create_requires_auth_and_returns_a_persisted_ticket() {
    let app = app();
    let token = account(&app, "mert@example.com").await;
    assert_eq!(
        request(&app, "POST", "/api/tickets", json!({"title":"Hi"}), None)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let (status, ticket) = request(
        &app,
        "POST",
        "/api/tickets",
        json!({"title":"  Ship API  "}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{ticket}");
    assert_eq!(ticket["title"], "Ship API");
    assert_eq!(ticket["status"], "open");
    assert!(ticket["id"].as_i64().unwrap() > 0);
}
