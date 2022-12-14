mod common;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
#[tokio::test]
async fn malformed_or_duplicate_authorization_is_rejected() {
    let app = common::app();
    for value in [
        "",
        "Basic abc",
        "Bearer",
        "Bearer  abc",
        "Bearer abc def",
        "bearer forged",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/me")
                    .header("authorization", value)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{value}");
    }
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/me")
                .header("authorization", "Bearer one")
                .header("authorization", "Bearer two")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
