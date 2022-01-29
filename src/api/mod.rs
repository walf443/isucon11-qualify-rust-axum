use std::error::Error;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::log;

pub mod isu;
pub mod isu_condition;

pub async fn post_initialize() -> Result<impl IntoResponse, (StatusCode, String)> {
    let status = tokio::process::Command::new("./sql/init.sh")
        .status().await
        .map_err(|e| {
            log::error!("exec init.sh error");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if !status.success() {
        log::error!("exec init.sh failed with exit code {:?}", status.code());
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "exec init.sh".to_string()));
    }
    Ok((StatusCode::OK, Json("OK")))
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
