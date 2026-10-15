mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn current_user_requires_a_valid_bearer_and_never_returns_hashes() {
    let app = app();
    register(&app, "mert@example.com").await;
    assert_eq!(
        request(&app, "GET", "/api/me", Value::Null, None).await.0,
        StatusCode::UNAUTHORIZED
    );
    let (_, login) = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"mert@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    let token = login["access_token"].as_str().unwrap();
    let (status, user) = request(&app, "GET", "/api/me", Value::Null, Some(token)).await;
    assert_eq!(status, StatusCode::OK, "{user}");
    assert_eq!(user["email"], "mert@example.com");
    assert!(user.get("password_hash").is_none());
    assert!(user.get("token_version").is_none());
    assert_eq!(
        request(&app, "GET", "/api/me", Value::Null, Some("forged"))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}
