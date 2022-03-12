use crate::models::isu::Isu;
use crate::models::isu_condition::IsuCondition;
use crate::models::user::UserID;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;
use crate::repos::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type IsuWithCondition = (Isu, Option<IsuCondition>);

pub struct IsuListService<R: RepositoryManager> {
    repo: Arc<R>,
}

impl<R: RepositoryManager> IsuListService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn run(&self, jia_user_id: &UserID) -> Result<Vec<IsuWithCondition>> {
        let chairs = self
            .repo
            .isu_repository()
            .find_all_by_user_id(jia_user_id)
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
    use crate::models::isu::{Isu, IsuID, IsuUUID};
    use crate::models::isu_condition::{IsuCondition, IsuConditionID};
    use crate::models::user::UserID;
    use crate::repos;
    use crate::repos::repository_manager::tests::MockRepositoryManager;
    use crate::repos::Result;
    use crate::services::isu_list_service::IsuListService;
    use chrono::NaiveDateTime;
    use std::sync::Arc;

    #[tokio::test]
    async fn with_empty_list() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_user_id()
            .returning(|_user_id| Ok(vec![]));

        let service = IsuListService::new(Arc::new(repo));
        let result = service.run(&UserID::new("test".to_string())).await?;
        assert_eq!(result.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn with_got_error_when_getting_list() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_user_id()
            .returning(|_user_id| Err(repos::Error::TestError()));

        let service = IsuListService::new(Arc::new(repo));
        let result = service.run(&UserID::new("test".to_string())).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn with_items() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_user_id()
            .returning(|_user_id| {
                Ok(vec![
                    Isu {
                        id: IsuID::new(1),
                        jia_user_id: UserID::new("1".to_string()),
                        jia_isu_uuid: IsuUUID::new("test".to_string()),
                        name: "".to_string(),
                        character: None,
                    },
                    Isu {
                        id: IsuID::new(2),
                        jia_user_id: UserID::new("2".to_string()),
                        jia_isu_uuid: IsuUUID::new("test2".to_string()),
                        name: "".to_string(),
                        character: None,
                    },
                ])
            });

        repo.isu_condition_repository
            .expect_find_last_by_isu_id()
            .returning(|uuid| {
                if uuid == &IsuUUID::new("test2".to_string()) {
                    Ok(Some(IsuCondition {
                        id: IsuConditionID::new("1".to_string()),
                        jia_isu_uuid: IsuUUID::new("test2".to_string()),
                        is_sitting: false,
                        condition: "".to_string(),
                        message: "".to_string(),
                        timestamp: NaiveDateTime::from_timestamp(61, 0),
                        created_at: None,
                    }))
                } else {
                    Ok(None)
                }
            });

        let service = IsuListService::new(Arc::new(repo));
        let result = service.run(&UserID::new("test".to_string())).await?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].1, None);
        assert_ne!(result[1].1, None);

        Ok(())
    }
}
