use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
