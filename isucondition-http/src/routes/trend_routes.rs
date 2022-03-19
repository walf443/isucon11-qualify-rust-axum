use crate::responses::error::Error;
use crate::responses::trend_response::TrendResponse;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::services::service_manager::ServiceManager;
use isucondition_core::services::trend_list_service::TrendListService;
use std::sync::Arc;

pub async fn get_trends<Service: ServiceManager>(
    Extension(service): Extension<Arc<Service>>,
) -> Result<impl IntoResponse, Error> {
    let trends = service.trend_list_service().run().await?;
    let trends: Vec<TrendResponse> = trends.into_iter().map(|t| t.into()).collect();

    Ok(Json(trends))
}

#[cfg(test)]
mod tests {
    mod get_trends {
        use crate::{get_trends, responses};
        use axum::extract::Extension;
        use axum::response::IntoResponse;
        use isucondition_core::models::trend::Trend;
        use isucondition_core::repos::repository_manager::tests::MockRepositoryManager;
        use isucondition_core::repos::Error::TestError;
        use isucondition_core::services::service_manager::tests::MockServiceManager;
        use std::sync::Arc;

        #[tokio::test]
        async fn success() -> Result<(), responses::error::Error> {
            let repo = MockRepositoryManager::new();
            let mut service = MockServiceManager::new(Arc::new(repo));

            service.trend_list_service.expect_run().returning(|| {
                Ok(vec![Trend {
                    character: "".to_string(),
                    info: vec![],
                    warning: vec![],
                    critical: vec![],
                }])
            });

            let ext_service = Extension(Arc::new(service));
            let res = get_trends(ext_service).await?;
            assert_eq!(res.into_response().status(), 200);

            Ok(())
        }

        #[tokio::test]
        #[should_panic(expected = "ReposError")]
        async fn trend_list_service_fail() {
            let repo = MockRepositoryManager::new();
            let mut service = MockServiceManager::new(Arc::new(repo));

            service
                .trend_list_service
                .expect_run()
                .returning(|| Err(TestError()));

            let ext_service = Extension(Arc::new(service));
            get_trends(ext_service).await.unwrap();
        }
    }
}
