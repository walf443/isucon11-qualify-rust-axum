use crate::repos;
use crate::repos::repository_manager::tests::MockRepositoryManager;
use crate::services::trend_list_service::{TrendListService, TrendListServiceImpl};
use repos::Result;
use std::sync::Arc;

#[tokio::test]
async fn with_empty_character() -> Result<()> {
    let mut repo = MockRepositoryManager::new();
    repo.isu_repository
        .expect_find_character_group_by()
        .returning(|| Ok(Vec::new()));

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let result = service.run().await?;

    assert_eq!(result.len(), 0);

    Ok(())
}
