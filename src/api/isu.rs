use axum::extract::Path;
use axum::Json;
use crate::{IntoResponse, StatusCode};

pub async fn get_isu_list() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn post_isu() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_isu_id(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_isu_icon(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_isu_graph(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

