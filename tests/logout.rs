mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn logout_is_idempotent_and_revokes_the_refresh_session() {
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
    let payload = json!({"refresh_token":login["refresh_token"]});
    for _ in 0..2 {
        assert_eq!(
            request(&app, "POST", "/api/auth/logout", payload.clone(), None)
                .await
                .0,
            StatusCode::NO_CONTENT
        );
    }
    assert_eq!(
        request(&app, "POST", "/api/auth/refresh", payload, None)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}
