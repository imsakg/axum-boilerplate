mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn email_verification_requires_a_single_use_challenge() {
    let state = state();
    let db = state.db.clone();
    let app = axum_boilerplate::app(state);
    register(&app, "mert@example.com").await;
    let (_, login) = request(
        &app,
        "POST",
        "/api/auth/login",
        json!({"email":"mert@example.com","password":"a correct horse battery"}),
        None,
    )
    .await;
    let bearer = login["access_token"].as_str();
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/request-verification",
            Value::Null,
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let (status, response) = request(
        &app,
        "POST",
        "/api/auth/request-verification",
        Value::Null,
        bearer,
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert!(response.get("token").is_none());
    let token = db
        .with(|c| {
            Ok(
                c.query_row("SELECT token FROM outbox WHERE kind='verify'", [], |r| {
                    r.get::<_, String>(0)
                })?,
            )
        })
        .unwrap();
    let payload = json!({"token":token});
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/auth/verify-email",
            payload.clone(),
            None
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        request(&app, "POST", "/api/auth/verify-email", payload, None)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(&app, "GET", "/api/me", Value::Null, bearer).await.1["verified"],
        true
    );
}
