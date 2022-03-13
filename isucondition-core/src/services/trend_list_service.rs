use crate::models::trend::Trend;
use crate::repos;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::services::trend_service::TrendService;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait TrendListService<R: 'static + RepositoryManager> {
    fn from_repo(repo: Arc<R>) -> Self;
    async fn run(&self) -> repos::Result<Vec<Trend>>;
}

#[derive(Clone)]
pub struct TrendListServiceImpl<R: RepositoryManager> {
    repo: Arc<R>,
}

#[async_trait]
impl<R: 'static + RepositoryManager> TrendListService<R> for TrendListServiceImpl<R> {
    fn from_repo(repo: Arc<R>) -> Self {
        Self { repo }
    }

    async fn run(&self) -> repos::Result<Vec<Trend>> {
        let character_list = self.repo.isu_repository().find_character_group_by().await?;

        let mut trends: Vec<Trend> = Vec::new();

        for character in character_list {
            if !character.is_none() {
                let character = character.unwrap();
                let trend_service = TrendService::new(self.repo.as_ref());
                let trend = trend_service.run(character).await?;

                trends.push(trend);
            }
        }

        Ok(trends)
    }
}

#[cfg(test)]
mod tests {
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
}
