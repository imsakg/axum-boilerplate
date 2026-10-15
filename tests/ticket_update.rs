mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn patch_updates_title_and_status_without_changing_id() {
    let app = app();
    let token = account(&app, "mert@example.com").await;
    let (_, ticket) = request(
        &app,
        "POST",
        "/api/tickets",
        json!({"title":"Before"}),
        Some(&token),
    )
    .await;
    let path = format!("/api/tickets/{}", ticket["id"]);
    let (status, updated) = request(
        &app,
        "PATCH",
        &path,
        json!({"title":" After ","status":"closed"}),
        Some(&token),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{updated}");
    assert_eq!(updated["id"], ticket["id"]);
    assert_eq!(updated["title"], "After");
    assert_eq!(updated["status"], "closed");
    assert_eq!(
        request(&app, "PATCH", &path, json!({}), Some(&token))
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &app,
            "PATCH",
            &path,
            json!({"status":"invalid"}),
            Some(&token)
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
}
