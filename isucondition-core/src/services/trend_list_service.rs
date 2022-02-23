use crate::models::trend::Trend;
use crate::repos;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::services::trend_service::TrendService;

pub struct TrendListService<'r, R: RepositoryManager> {
    repo: &'r R,
}

impl<'r, R: RepositoryManager> TrendListService<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }

    pub async fn run(&self) -> repos::Result<Vec<Trend>> {
        let character_list = self.repo.isu_repository().find_character_group_by().await?;

        let mut trends: Vec<Trend> = Vec::new();

        for character in character_list {
            if !character.is_none() {
                let character = character.unwrap();
                let trend_service = TrendService::new(self.repo);
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
    use crate::services::trend_list_service::TrendListService;
    use repos::Result;

    #[tokio::test]
    async fn with_empty_character() -> Result<()> {
        let mut repo = MockRepositoryManager::new();
        repo.isu_repository
            .expect_find_character_group_by()
            .returning(|| Ok(Vec::new()));

        let service = TrendListService::new(&repo);
        let result = service.run().await?;

        assert_eq!(result.len(), 0);

        Ok(())
    }
}
