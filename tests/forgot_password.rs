mod common;
use axum::http::StatusCode;
use common::*;
use serde_json::json;
#[tokio::test]
async fn forgot_password_does_not_disclose_account_existence_or_reset_token() {
    let state = state();
    let db = state.db.clone();
    let app = axum_boilerplate::app(state);
    register(&app, "mert@example.com").await;
    let a = request(
        &app,
        "POST",
        "/api/auth/forgot-password",
        json!({"email":"mert@example.com"}),
        None,
    )
    .await;
    let b = request(
        &app,
        "POST",
        "/api/auth/forgot-password",
        json!({"email":"unknown@example.com"}),
        None,
    )
    .await;
    assert_eq!(a, b);
    assert_eq!(a.0, StatusCode::ACCEPTED);
    assert!(a.1.get("token").is_none());
    db.with(|c| {
        let (token,hash):(String,String)=c.query_row("SELECT o.token,c.hash FROM outbox o JOIN challenges c ON o.user_id=c.user_id AND o.kind=c.kind",[],|r| Ok((r.get(0)?,r.get(1)?)))?;
        assert_ne!(token,hash); assert_eq!(axum_boilerplate::crypto::fingerprint(&token),hash); Ok(())
    }).unwrap();
}
