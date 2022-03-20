use crate::responses;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::models::isu_association_config::IsuAssociationConfigForm;
use isucondition_core::repos::isu_association_config_repository::IsuAssociationConfigRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::services::reset_database_service::ResetDatabaseService;
use isucondition_core::services::service_manager::ServiceManager;
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

pub async fn post_initialize<Repo: RepositoryManager, Service: ServiceManager>(
    Extension(repo): Extension<Arc<Repo>>,
    Extension(service): Extension<Arc<Service>>,
    Json(payload): Json<PostInitializeRequest>,
) -> Result<impl IntoResponse, responses::error::Error> {
    service.reset_database_service().run().await?;

    let form = IsuAssociationConfigForm::build(
        "jia_service_url".to_string(),
        payload.jia_service_url.to_string(),
    )?;

    repo.isu_association_config_repository()
        .insert(&form)
        .await?;

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

        assert_eq!(StatusCode::UNSUPPORTED_MEDIA_TYPE, res.status());

        Ok(())
    }
}
