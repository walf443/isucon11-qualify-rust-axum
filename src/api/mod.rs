use axum::http::StatusCode;
use axum::{extract, Json};
use axum::response::IntoResponse;
use sqlx::{MySqlPool};
use tracing::log;
use serde::Serialize;
use serde::Deserialize;
use crate::model::isu_association_config_repository::{IsuAssociationConfigRepository, IsuAssociationConfigRepositoryImpl};

#[cfg(test)]
use crate::test_helper;

pub mod isu;
pub mod isu_condition;

#[derive(Serialize,Deserialize)]
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

#[tokio::test]
async fn test_post_initialize() -> anyhow::Result<()> {
    std::env::set_var("MYSQL_DBNAME", std::env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned() ));
    let app = test_helper::spawn_app().await;
    let client = reqwest::Client::new();
    let res = client.post(app.url.join("/initialize").unwrap()).send().await
        .expect("Failed to request");

    assert!(res.status().is_success());
    let json = res.json::<PostInitializeResponse>().await?;
    assert_eq!("rust", json.language);

    let result = sqlx::query!("SELECT COUNT(*) as count from isu_association_config").fetch_one(&app.database).await;
    assert_eq!(1, result.unwrap().count, "isu_association_config record created");

    Ok(())
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

#[tokio::test]
async fn test_get_index() {
    let app = test_helper::spawn_app().await;
    let client = reqwest::Client::new();
    let res = client.get(app.url.join("/").unwrap()).send().await
        .expect("Failed to request");

    assert!(res.status().is_success());
}

pub async fn post_authentication() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
