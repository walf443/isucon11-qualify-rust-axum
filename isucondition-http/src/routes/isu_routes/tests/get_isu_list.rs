use crate::get_isu_list;
use crate::requests::current_user_id::CurrentUserID;
use crate::responses::error::Error;
use axum::extract::Extension;
use isucondition_core::models::user::UserID;
use isucondition_core::repos::repository_manager::tests::MockRepositoryManager;
use std::sync::Arc;

#[tokio::test]
#[should_panic(expected = "UnauthorizedError")]
async fn no_sigined_in() -> () {
    let repo = MockRepositoryManager::new();
    let ext_repo = Extension(Arc::new(repo));
    get_isu_list(ext_repo, CurrentUserID::None).await.unwrap();
}

#[tokio::test]
async fn signed_in() -> Result<(), anyhow::Error> {
    let mut repo = MockRepositoryManager::new();
    let user_id = UserID::new("1".to_string());
    repo.isu_repository
        .expect_find_all_by_user_id()
        .returning(|_user_id| Ok(Vec::new()));

    let ext_repo = Extension(Arc::new(repo));
    let result = get_isu_list(ext_repo, CurrentUserID::Some(user_id.clone())).await;

    assert!(result.is_ok());

    Ok(())
}
