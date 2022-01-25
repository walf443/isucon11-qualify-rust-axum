use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

pub mod isu;
pub mod isu_condition;

pub async fn post_initialize() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn post_signout() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_me() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn post_authentication() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
