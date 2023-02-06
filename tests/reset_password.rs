mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn reset_consumes_the_challenge_and_invalidates_old_credentials() {
    let state = state();
    let db = state.db.clone();
    let app = axum_boilerplate::app(state);
    let user = register(&app, "mert@example.com").await;
    let (_, login) = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"mert@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    let challenge = "a".repeat(64);
    db.issue_challenge(
        user["id"].as_i64().unwrap(),
        "reset",
        &challenge,
        axum_boilerplate::crypto::now() + 300,
    )
    .unwrap();
    let payload = json!({"token":challenge,"password":"a different horse battery"});
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/reset-password",
            payload.clone(),
            None
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        request(&app, "POST", "/api/auth/reset-password", payload, None)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &app,
            "GET",
            "/api/me",
            Value::Null,
            login["access_token"].as_str()
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
            json!({"email":"mert@example.com","password":"a different horse battery"}),
            None
        )
        .await
        .0,
        StatusCode::OK
    );
}
