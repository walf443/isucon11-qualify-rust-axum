use axum::http::StatusCode;
use axum::{extract, Json};
use axum::response::IntoResponse;
use sqlx::{MySqlPool};
use tracing::log;
use serde::Serialize;
use crate::model::isu_association_config_repository::{IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl};

pub mod isu;
pub mod isu_condition;

#[derive(Serialize)]
pub struct PostInitializeResponse {
    pub language: String,
}

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

    let isu_association_config_repo = IsuAssociationConfigRepositoryImpl {
        pool: pool.0,
    };
    isu_association_config_repo.insert("jia_service_url", "http://localhost:3000").await.map_err(|e| {
        log::error!("insert isu_association_config error");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok((StatusCode::OK, Json(PostInitializeResponse {
        language: "rust".to_string(),
    })))
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
