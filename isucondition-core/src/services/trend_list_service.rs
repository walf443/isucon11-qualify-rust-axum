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
