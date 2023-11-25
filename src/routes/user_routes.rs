use axum::{
    extract::{Path, Query},
    Json,
};

pub async fn get_users(Path(id): Path<i32>) -> String {
    return id.to_string();
}
