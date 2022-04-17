use crate::get_isu_list;
use crate::requests::current_user_id::CurrentUserID;
use axum::extract::Extension;
use isucondition_core::models::user::UserID;
use isucondition_core::repos;
use isucondition_core::repos::repository_manager::tests::MockRepositoryManager;
use isucondition_core::services::service_manager::tests::MockServiceManager;
use std::sync::Arc;

#[tokio::test]
#[should_panic(expected = "UnauthorizedError")]
async fn no_sigined_in() -> () {
    let repo = Arc::new(MockRepositoryManager::new());
    let service = MockServiceManager::new(repo);
    let ext_service = Extension(Arc::new(service));
    get_isu_list(ext_service, CurrentUserID::None)
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "ReposError")]
async fn signed_in_and_isu_list_service_returning_error() -> () {
    let repo = Arc::new(MockRepositoryManager::new());
    let mut service = MockServiceManager::new(repo);
    let user_id = UserID::new("1".to_string());

    service
        .isu_list_service
        .expect_run()
        .returning(|_user_id| Err(repos::Error::TestError()));

    let ext_service = Extension(Arc::new(service));
    get_isu_list(ext_service, CurrentUserID::Some(user_id.clone()))
        .await
        .unwrap();
}

#[tokio::test]
async fn signed_in_and_isu_list_service_returning_empty() -> Result<(), anyhow::Error> {
    let repo = Arc::new(MockRepositoryManager::new());
    let mut service = MockServiceManager::new(repo);
    let user_id = UserID::new("1".to_string());

    service
        .isu_list_service
        .expect_run()
        .returning(|_user_id| Ok(Vec::new()));

    let ext_service = Extension(Arc::new(service));
    let result = get_isu_list(ext_service, CurrentUserID::Some(user_id.clone())).await;

    assert!(result.is_ok());

    Ok(())
}
