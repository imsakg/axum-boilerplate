//! A small Axum service with an embeddable router for tests.
pub mod error;
pub mod model;
pub mod web;
pub use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    response::Html,
    routing::{get, get_service},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

/// Construct the router without binding a socket.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/hello2/:name", get(hello_name))
        .merge(web::routes_login::routes())
        .layer(CookieManagerLayer::new())
        .fallback_service(get_service(ServeDir::new("./")))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello(Query(params): Query<HelloParams>) -> Html<String> {
    Html(format!(
        "Hello <strong>{}!</strong>",
        params.name.as_deref().unwrap_or("World")
    ))
}
async fn hello_name(Path(name): Path<String>) -> Html<String> {
    Html(format!("Hello2 <strong>{name}!</strong>"))
}
