#![allow(dead_code)]
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use axum_boilerplate::{config::Config, db::Database, state::AppState};
use serde_json::{json, Value};
use tower::ServiceExt;

pub fn state() -> AppState {
    let config = Config::from_values(&std::collections::HashMap::from([(
        "JWT_SECRET".into(),
        "test-only-32-byte-secret-not-for-production".into(),
    )]))
    .unwrap();
    AppState::new(config, Database::open(":memory:").unwrap())
}
pub fn app() -> Router {
    axum_boilerplate::app(state())
}
pub async fn request(
    app: &Router,
    method: &str,
    uri: &str,
    body: Value,
    token: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({"raw":String::from_utf8_lossy(&bytes)}))
    };
    (status, value)
}
pub async fn login(app: &Router, email: &str) -> Value {
    let (status, body) = request(
        app,
        "POST",
        "/api/auth/login",
        json!({"email":email,"password":"a correct horse battery"}),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}
pub async fn account(app: &Router, email: &str) -> String {
    register(app, email).await;
    login(app, email).await["access_token"]
        .as_str()
        .unwrap()
        .to_owned()
}

pub async fn register(app: &Router, email: &str) -> Value {
    let (status, body) = request(
        app,
        "POST",
        "/api/auth/register",
        json!({"email":email,"password":"a correct horse battery"}),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    body
}
