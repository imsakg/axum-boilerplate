#![allow(dead_code)]

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    /* #region: Start Server */
    let addr = SocketAddr::from(([127, 0, 0, 1], 5353));
    println!("--> Server running on: http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    /* #endregion */
}

/* #region: Routes Hello */
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2)) // :name tells axum where to data extract from
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// /hello?name=msa
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}!</strong>"))
}

// /hello2/msa
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}!</strong>"))
}

/* #endregion */
