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
    let addr_str = format!(
        "{}:{}",
        DEFAULT_SERVICE_LISTEN_INTERFACE, DEFAULT_SERVICE_LISTEN_PORT
    );
    let addr = SocketAddr::from_str(&addr_str)
        .expect(&format!("Could not create listening address: {}", addr_str));

    println!("listening on {:?}", addr_str);

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/healthcheck", get(healthcheck_handler))
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn index_handler_test() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        dbg!(&body);
        assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
    }

    #[tokio::test]
    async fn healthcheck_handler_test() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/healthcheck")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        dbg!(&body);
        assert_eq!(&body[..], b"ok");
    }
}
