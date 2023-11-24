use serde::Deserialize;
use std::{fmt::format, net::SocketAddr};
use tower_http::services::ServeDir;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};

mod error;

#[tokio::main]
async fn main() {
    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // region: -- Start Server
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap()

    // endredgion: -- Start Server
}

fn routes_static() -> Router {
    return Router::new().nest_service("/", get_service(ServeDir::new("./")));
}

fn routes_hello() -> Router {
    return Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(">> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name: &str = params.name.as_deref().unwrap_or("default");
    Html(format!("Hello <strong>{name}</strong"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(">> {:<12} - handler_hello 2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
