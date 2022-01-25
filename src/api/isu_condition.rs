use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

pub async fn get_isu_conditions(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn post_isu_condition(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

