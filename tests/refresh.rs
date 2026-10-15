mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn refresh_rotates_and_rejects_replay() {
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
    let old = login["refresh_token"]
        .as_str()
        .expect("login must return a refresh token");
    let (status, next) = request(
        &app,
        "POST",
        "/api/auth/refresh",
        json!({"refresh_token":old}),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{next}");
    assert_ne!(next["refresh_token"], old);
    assert_ne!(next["access_token"], login["access_token"]);
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/refresh",
            json!({"refresh_token":old}),
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
            "/api/auth/refresh",
            json!({"refresh_token":next["refresh_token"]}),
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn invalid_refresh_is_unauthorized() {
    let app = app();
    for token in ["", "forged", &"x".repeat(65)] {
        assert_eq!(
            request(
                &app,
                "POST",
                "/api/auth/refresh",
                json!({"refresh_token":token}),
                None
            )
            .await
            .0,
            StatusCode::UNAUTHORIZED
        );
    }
}
