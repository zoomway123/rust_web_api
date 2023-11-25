mod hello_world;
mod user_routes;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::{hello_path, hello_post, hello_world, query_params};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/hello", post(hello_post))
        .route("/hello/:id", get(hello_path))
        .route("/queryParams/", post(query_params))
}
