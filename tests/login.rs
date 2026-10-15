mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn login_returns_an_access_token() {
    let app = app();
    register(&app, "mert@example.com").await;
    let (status, body) = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"MERT@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["expires_in"], 900);
    assert_eq!(body["access_token"].as_str().unwrap().split('.').count(), 3);
    assert!(body.get("password_hash").is_none());
}
#[tokio::test]
async fn missing_and_wrong_password_have_the_same_response() {
    let app = app();
    register(&app, "mert@example.com").await;
    let a = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"mert@example.com","password":"wrong password"}),
        None,
    )
    .await;
    let b = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"missing@example.com","password":"wrong password"}),
        None,
    )
    .await;
    assert_eq!(a, b);
    assert_eq!(a.0, StatusCode::UNAUTHORIZED);
}
