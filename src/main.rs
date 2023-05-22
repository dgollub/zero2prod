//! Run with
//!
//! ```not_rust
//! cargo run
//! ```

use axum::{response::Html, routing::get, Router};
use std::{net::SocketAddr, str::FromStr};

mod constants;
use constants::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/healthcheck", get(healthcheck_handler));

    let addr_str = format!(
        "{}:{}",
        DEFAULT_SERVICE_LISTEN_INTERFACE, DEFAULT_SERVICE_LISTEN_PORT
    );
    let addr = SocketAddr::from_str(&addr_str)
        .expect(&format!("Could not create listening address: {}", addr_str));

    println!("listening on {:?}", addr_str);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}
