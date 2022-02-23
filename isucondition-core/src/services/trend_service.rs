use crate::models::isu_condition::ConditionLevel;
use crate::models::trend::{IsuWithCondition, Trend};
use crate::repos;
use crate::repos::isu_condition_repository::IsuConditionRepository;
use crate::repos::isu_repository::IsuRepository;
use crate::repos::repository_manager::RepositoryManager;

pub struct TrendService<'r, R: RepositoryManager> {
    repo: &'r R,
}

impl<'r, R: RepositoryManager> TrendService<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, character: String) -> repos::Result<Trend> {
        let isu_list = self
            .repo
            .isu_repository()
            .find_all_by_character(&character)
            .await?;

        let mut info_conditions: Vec<IsuWithCondition> = Vec::new();
        let mut warning_conditions: Vec<IsuWithCondition> = Vec::new();
        let mut critical_conditions: Vec<IsuWithCondition> = Vec::new();

        for isu in isu_list {
            let condition = self
                .repo
                .isu_condition_repository()
                .find_last_by_isu_id(&isu.jia_isu_uuid)
                .await?;

            match condition {
                Some(last_isu_condition) => {
                    let level = last_isu_condition.condition_level();

                    match level {
                        ConditionLevel::Info => info_conditions.push((isu, last_isu_condition)),
                        ConditionLevel::Warning => {
                            warning_conditions.push((isu, last_isu_condition))
                        }
                        ConditionLevel::Critical => {
                            critical_conditions.push((isu, last_isu_condition))
                        }
                        ConditionLevel::Unknown => {}
                    };
                }
                None => (),
            };
        }

        info_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));
        warning_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));
        critical_conditions.sort_by_key(|(_, cond)| std::cmp::Reverse(cond.timestamp));

        Ok(Trend {
            character,
            info: info_conditions,
            warning: warning_conditions,
            critical: critical_conditions,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::isu::{Isu, IsuID, IsuUUID};
    use crate::models::isu_condition::{IsuCondition, IsuConditionID};
    use crate::models::user::UserID;
    use crate::repos;
    use crate::repos::repository_manager::tests::MockRepositoryManager;
    use crate::services::trend_service::TrendService;
    use chrono::NaiveDateTime;
    use repos::Result;

    #[tokio::test]
    async fn with_isu_empty() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_character()
            .returning(|_| Ok(Vec::new()));

        let service = TrendService::new(&repo);
        let trend = service.run("test".to_string()).await?;
        assert_eq!(trend.character, "test".to_string());
        assert_eq!(trend.info.len(), 0);
        assert_eq!(trend.warning.len(), 0);
        assert_eq!(trend.critical.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn with_isu_found_but_condition_not_exist() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_character()
            .returning(|_| {
                Ok(vec![Isu {
                    id: IsuID::new(1),
                    jia_user_id: UserID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    name: "".to_string(),
                    character: Some("test".to_string()),
                }])
            });

        repo.isu_condition_repository
            .expect_find_last_by_isu_id()
            .returning(|_| Ok(None));

        let service = TrendService::new(&repo);
        let trend = service.run("test".to_string()).await?;
        assert_eq!(trend.character, "test".to_string());
        assert_eq!(trend.info.len(), 0);
        assert_eq!(trend.warning.len(), 0);
        assert_eq!(trend.critical.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn with_info_condition() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_character()
            .returning(|_| {
                Ok(vec![Isu {
                    id: IsuID::new(1),
                    jia_user_id: UserID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    name: "".to_string(),
                    character: Some("test".to_string()),
                }])
            });

        repo.isu_condition_repository
            .expect_find_last_by_isu_id()
            .returning(|_| {
                let condition = IsuCondition {
                    id: IsuConditionID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    is_sitting: false,
                    condition: "".to_string(),
                    message: "".to_string(),
                    timestamp: NaiveDateTime::from_timestamp(0, 0),
                    created_at: None,
                };

                Ok(Some(condition))
            });

        let service = TrendService::new(&repo);
        let trend = service.run("test".to_string()).await?;
        assert_eq!(trend.character, "test".to_string());
        assert_eq!(trend.info.len(), 1);
        assert_eq!(trend.warning.len(), 0);
        assert_eq!(trend.critical.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn with_warning_condition() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_character()
            .returning(|_| {
                Ok(vec![Isu {
                    id: IsuID::new(1),
                    jia_user_id: UserID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    name: "".to_string(),
                    character: Some("test".to_string()),
                }])
            });

        repo.isu_condition_repository
            .expect_find_last_by_isu_id()
            .returning(|_| {
                let condition = IsuCondition {
                    id: IsuConditionID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    is_sitting: false,
                    condition: "hoghoge=true".to_string(),
                    message: "".to_string(),
                    timestamp: NaiveDateTime::from_timestamp(0, 0),
                    created_at: None,
                };

                Ok(Some(condition))
            });

        let service = TrendService::new(&repo);
        let trend = service.run("test".to_string()).await?;
        assert_eq!(trend.character, "test".to_string());
        assert_eq!(trend.info.len(), 0);
        assert_eq!(trend.warning.len(), 1);
        assert_eq!(trend.critical.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn with_critical_condition() -> Result<()> {
        let mut repo = MockRepositoryManager::new();

        repo.isu_repository
            .expect_find_all_by_character()
            .returning(|_| {
                Ok(vec![Isu {
                    id: IsuID::new(1),
                    jia_user_id: UserID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    name: "".to_string(),
                    character: Some("test".to_string()),
                }])
            });

        repo.isu_condition_repository
            .expect_find_last_by_isu_id()
            .returning(|_| {
                let condition = IsuCondition {
                    id: IsuConditionID::new("1".to_string()),
                    jia_isu_uuid: IsuUUID::new("test".to_string()),
                    is_sitting: false,
                    condition: "foo=true,bar=true,baz=true".to_string(),
                    message: "".to_string(),
                    timestamp: NaiveDateTime::from_timestamp(0, 0),
                    created_at: None,
                };

                Ok(Some(condition))
            });

        let service = TrendService::new(&repo);
        let trend = service.run("test".to_string()).await?;
        assert_eq!(trend.character, "test".to_string());
        assert_eq!(trend.info.len(), 0);
        assert_eq!(trend.warning.len(), 0);
        assert_eq!(trend.critical.len(), 1);

        Ok(())
    }
}
