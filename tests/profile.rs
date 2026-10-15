mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn profile_update_is_scoped_and_validated() {
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
    let (status, user) = request(
        &app,
        "PATCH",
        "/api/me",
        json!({"display_name":"  Mert  "}),
        Some(token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{user}");
    assert_eq!(user["display_name"], "Mert");
    assert_eq!(
        request(
            &app,
            "PATCH",
            "/api/me",
            json!({"display_name":"x".repeat(81)}),
            Some(token)
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &app,
            "PATCH",
            "/api/me",
            json!({"display_name":"other"}),
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
}
