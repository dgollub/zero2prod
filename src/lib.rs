use axum::{
    response::Html,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::{net::SocketAddr, str::FromStr};

mod constants;
use constants::*;

// TODO(dkg): is there a more generic type that could be used instead? Or trait?
type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn create_server() -> Result<AppServer, hyper::Error> {
    let addr_str = format!(
        "{}:{}",
        DEFAULT_SERVICE_LISTEN_INTERFACE, DEFAULT_SERVICE_LISTEN_PORT
    );
    let addr = SocketAddr::from_str(&addr_str)
        .expect(&format!("Could not create listening address: {}", addr_str));

    println!("listening on {:?}", addr_str);

    let router = create_router();
    let server = axum::Server::bind(&addr).serve(router.into_make_service());

    Ok(server)
}

pub fn create_router() -> Router {
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
