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
    let form = IsuAssociationConfigForm::build(
        "jia_service_url".to_string(),
        payload.jia_service_url.to_string(),
    )?;

    service.reset_database_service().run().await?;

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
    use crate::post_initialize;
    use crate::responses::error::Error;
    use crate::routes::initialize_routes::PostInitializeRequest;
    use axum::extract::Extension;
    use axum::Json;
    use isucondition_core::repos::repository_manager::tests::MockRepositoryManager;
    use isucondition_core::repos::Error::{CommandExecutionError, TestError};
    use isucondition_core::services::service_manager::tests::MockServiceManager;
    use std::sync::Arc;

    #[tokio::test]
    #[should_panic(expected = "HttpUrlParseError")]
    async fn invalid_jia_service_url() {
        let repo = Arc::new(MockRepositoryManager::new());
        let service = MockServiceManager::new(repo.clone());

        let res = post_initialize(
            Extension(repo),
            Extension(Arc::new(service)),
            Json(PostInitializeRequest {
                jia_service_url: "javascript://alert('hoge')".to_string(),
            }),
        )
        .await;

        res.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "CommandExecutionError")]
    async fn reset_database_fail() {
        let repo = Arc::new(MockRepositoryManager::new());
        let mut service = MockServiceManager::new(repo.clone());
        service
            .reset_database_service
            .expect_run()
            .returning(|| Err(CommandExecutionError()));

        let res = post_initialize(
            Extension(repo),
            Extension(Arc::new(service)),
            Json(PostInitializeRequest {
                jia_service_url: "http://localhost:3000".to_string(),
            }),
        )
        .await;

        res.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ReposError")]
    async fn isu_association_insert_fail() {
        let mut repo = MockRepositoryManager::new();
        repo.isu_association_config_repository
            .expect_insert()
            .returning(|_form| Err(TestError()));

        let repo = Arc::new(repo);
        let mut service = MockServiceManager::new(repo.clone());
        service
            .reset_database_service
            .expect_run()
            .returning(|| Ok(()));

        let res = post_initialize(
            Extension(repo),
            Extension(Arc::new(service)),
            Json(PostInitializeRequest {
                jia_service_url: "http://localhost:3000".to_string(),
            }),
        )
        .await;

        res.unwrap();
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut repo = MockRepositoryManager::new();
        repo.isu_association_config_repository
            .expect_insert()
            .returning(|_form| Ok(()));

        let repo = Arc::new(repo);
        let mut service = MockServiceManager::new(repo.clone());
        service
            .reset_database_service
            .expect_run()
            .returning(|| Ok(()));

        let res = post_initialize(
            Extension(repo),
            Extension(Arc::new(service)),
            Json(PostInitializeRequest {
                jia_service_url: "http://localhost:3000".to_string(),
            }),
        )
        .await;
        assert!(res.is_ok());

        Ok(())
    }
}
