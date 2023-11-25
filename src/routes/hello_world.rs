use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};

pub async fn hello_world() -> String {
    return "Hello world".to_owned();
}

pub async fn hello_post(Json(body): Json<Message>) -> Json<Response> {
    return Json(Response {
        message: body.message,
        message_from_server: String::from("Whoah bud"),
    });
}

pub async fn hello_path(Path(id): Path<i32>) -> String {
    return id.to_string();
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    return Json(query);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    message: String,
    message_from_server: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    message: String,
    id: i32,
}
