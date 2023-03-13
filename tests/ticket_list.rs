mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};
#[tokio::test]
async fn lists_are_stably_ordered_and_paginated_with_bounds() {
    let app = app();
    let token = account(&app, "mert@example.com").await;
    for title in ["first", "second", "third"] {
        request(
            &app,
            "POST",
            "/api/tickets",
            json!({"title":title}),
            Some(&token),
        )
        .await;
    }
    let (status, body) = request(
        &app,
        "GET",
        "/api/tickets?limit=1&offset=1",
        Value::Null,
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["items"].as_array().unwrap().len(), 1);
    assert_eq!(body["items"][0]["title"], "second");
    assert_eq!(body["limit"], 1);
    assert_eq!(body["offset"], 1);
    for query in ["limit=0", "limit=101", "offset=100001"] {
        assert_eq!(
            request(
                &app,
                "GET",
                &format!("/api/tickets?{query}"),
                Value::Null,
                Some(&token)
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
}
