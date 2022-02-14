use crate::models::isu::Isu;
use crate::models::isu_condition::IsuCondition;
use crate::models::user::UserID;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::repos::Result;

pub type IsuWithCondition = (Isu, Option<IsuCondition>);

pub struct IsuListService<'r, R: RepositoryManager> {
    repo: &'r R,
}

impl<'r, R: RepositoryManager> IsuListService<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, jia_user_id: UserID) -> Result<Vec<IsuWithCondition>> {
        let chairs = self
            .repo
            .isu_repository()
            .find_all_by_user_id(&jia_user_id)
            .await?;

        let mut list: Vec<IsuWithCondition> = Vec::new();

        for chair in chairs {
            let last_isu_condition = self
                .repo
                .isu_condition_repository()
                .find_last_by_isu_id(&chair.jia_isu_uuid)
                .await?;

            list.push((chair, last_isu_condition));
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::get_db_connection_for_test;
    use crate::models::user::UserID;
    use crate::repos;
    use crate::repos::repository_manager::tests::MockRepositoryManager;
    use crate::repos::repository_manager::RepositoryManager;
    use crate::repos::Result;
    use crate::services::isu_list_service::IsuListService;
    use std::borrow::BorrowMut;

    #[tokio::test]
    async fn test_isu_list_service_with_empty_list() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_user_id()
            .returning(|_user_id| Ok(vec![]));

        let service = IsuListService::new(&repo);
        let result = service.run(UserID::new("test".to_string())).await?;
        assert_eq!(result.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_isu_list_service_with_got_error_when_getting_list() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_user_id()
            .returning(|_user_id| Err(repos::Error::TestError()));

        let service = IsuListService::new(&repo);
        let result = service.run(UserID::new("test".to_string())).await;
        assert!(result.is_err());

        Ok(())
    }
}
