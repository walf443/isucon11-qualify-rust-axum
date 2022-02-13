use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::repos::isu_association_config_repository::IsuAssociationConfigRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tracing::log;

#[derive(Serialize, Deserialize)]
pub struct PostInitializeRequest {
    jia_service_url: String,
}

#[derive(Serialize, Deserialize)]
struct PostInitializeResponse {
    language: String,
}

pub async fn post_initialize<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
    Json(payload): Json<PostInitializeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let status = tokio::process::Command::new("../sql/init.sh")
        .status()
        .await
        .map_err(|e| {
            log::error!("exec init.sh error");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if !status.success() {
        log::error!("exec init.sh failed with exit code {:?}", status.code());
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "exec init.sh".to_string(),
        ));
    }

    repo.isu_association_config_repository()
        .insert("jia_service_url", &payload.jia_service_url)
        .await
        .map_err(|e| {
            log::error!("insert isu_association_config error");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((
        StatusCode::OK,
        Json(PostInitializeResponse {
            language: "rust".to_string(),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use crate::routes::initialize_routes::{PostInitializeRequest, PostInitializeResponse};
    use crate::test_helper;
    use axum::http::StatusCode;
    use isucondition_core::test::Cleaner;

    #[tokio::test]
    async fn test_post_initialize() -> anyhow::Result<()> {
        std::env::set_var(
            "MYSQL_DBNAME",
            std::env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned()),
        );
        let app = test_helper::spawn_app().await;
        let mut cleaner = Cleaner::new(app.database.clone());
        cleaner.prepare_table("isu_association_config").await?;
        let client = reqwest::Client::new();
        let res = client
            .post(app.url.join("/initialize").unwrap())
            .json(&PostInitializeRequest {
                jia_service_url: "http://localost:3000".to_string(),
            })
            .send()
            .await
            .expect("Failed to request");

        assert_eq!(StatusCode::OK, res.status());
        let json = res.json::<PostInitializeResponse>().await?;
        assert_eq!("rust", json.language);

        let result = sqlx::query!("SELECT COUNT(*) as count from isu_association_config")
            .fetch_one(&app.database)
            .await;
        assert_eq!(
            1,
            result.unwrap().count,
            "isu_association_config record created"
        );

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_post_initialize_with_empty_body() -> anyhow::Result<()> {
        std::env::set_var(
            "MYSQL_DBNAME",
            std::env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned()),
        );
        let app = test_helper::spawn_app().await;
        let client = reqwest::Client::new();
        let res = client
            .post(app.url.join("/initialize").unwrap())
            .send()
            .await
            .expect("Failed to request");

        assert_eq!(StatusCode::BAD_REQUEST, res.status());

        Ok(())
    }
}
