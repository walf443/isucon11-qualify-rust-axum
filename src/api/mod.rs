use axum::http::StatusCode;
use axum::{extract, Json};
use axum::response::IntoResponse;
use sqlx::{MySqlPool, Pool};
use sqlx::mysql::MySql;
use tracing::log;

pub mod isu;
pub mod isu_condition;

pub async fn post_initialize(pool: extract::Extension<MySqlPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
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

    sqlx::query!("INSERT INTO `isu_association_config` (`name`, `url`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `url` = VALUES(`url`)",
        "jia_service_url", "http://localhost:3000")
        .fetch_all(&pool.0).await.map_err(|e| {
        log::error!("insert isu_association_config error");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

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
