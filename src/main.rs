#![allow(dead_code)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!</strong>") }),
    );
    /* #region: Start Server */
    let addr = SocketAddr::from(([127, 0, 0, 1], 5353));
    println!("--> Server running on: http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    /* #endregion */

    println!("Hello, world!");
}
