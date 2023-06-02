use axum::{
    extract::Form,
    response::Html,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

mod constants;

// TODO(dkg): is there a more generic type that could be used instead? Or trait?
type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;

#[derive(Serialize, Deserialize, Debug)]
struct FormData {
    pub name: String,
    pub email: String,
}

pub fn create_server(listener: TcpListener) -> Result<AppServer, hyper::Error> {
    println!("listening on {:?}", listener);

    let router = create_router();
    let server = axum::Server::from_tcp(listener)?.serve(router.into_make_service());

    Ok(server)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .route("/subscriptions", post(subscriptions_handler))
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}

async fn subscriptions_handler(Form(_form_data): Form<FormData>) -> &'static str {
    "ok"
}
