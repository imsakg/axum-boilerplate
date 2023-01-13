mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn password_change_checks_credentials_and_revokes_existing_tokens() {
    let app = app();
    register(&app, "mert@example.com").await;
    let (_, login) = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"mert@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    let token = login["access_token"].as_str().unwrap();
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/change-password",
            json!({"current_password":"wrong password","new_password":"a different horse battery"}),
            Some(token)
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(request(&app,"POST","/api/auth/change-password",json!({"current_password":"a correct horse battery","new_password":"a different horse battery"}),Some(token)).await.0,StatusCode::NO_CONTENT);
    assert_eq!(
        request(&app, "GET", "/api/me", Value::Null, Some(token))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/refresh",
            json!({"refresh_token":login["refresh_token"]}),
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/login",
            json!({"email":"mert@example.com","password":"a correct horse battery"}),
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/login",
            json!({"email":"mert@example.com","password":"a different horse battery"}),
            None
        )
        .await
        .0,
        StatusCode::OK
    );
}
