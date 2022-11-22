mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn registration_normalizes_email_and_does_not_return_credentials() {
    let app = app();
    let user = register(&app, " Mert@Example.COM ").await;
    assert_eq!(user["email"], "mert@example.com");
    assert!(user.get("password_hash").is_none());
    assert!(user.get("password").is_none());
    let (status, _) = request(
        &app,
        "POST",
        "/api/auth/register",
        json!({"email":"mert@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}
#[tokio::test]
async fn registration_rejects_invalid_email_and_short_password() {
    let app = app();
    for body in [
        json!({"email":"invalid","password":"a correct horse battery"}),
        json!({"email":"mert@example.com","password":"short"}),
    ] {
        assert_eq!(
            request(&app, "POST", "/api/auth/register", body, None)
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
}
