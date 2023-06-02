//! Run with
//!
//! ```not_rust
//! cargo run
//! ```
use std::net::TcpListener;

use zero2prod::create_server;

mod constants;
use constants::*;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let addr_str = format!(
        "{}:{}",
        DEFAULT_SERVICE_LISTEN_INTERFACE, DEFAULT_SERVICE_LISTEN_PORT
    );
    let listener = TcpListener::bind(addr_str).expect("Could not create tcp listener");

    let app = create_server(listener)?;

    app.await
}

#[cfg(test)]
mod tests {
    use zero2prod::create_router;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn index_handler_test() {
        let app = create_router();

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
        let app = create_router();

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
