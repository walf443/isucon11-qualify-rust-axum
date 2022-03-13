use crate::models::isu::{Isu, IsuID, IsuUUID};
use crate::models::isu_condition::{IsuCondition, IsuConditionID};
use crate::models::user::UserID;
use crate::repos;
use crate::repos::repository_manager::tests::MockRepositoryManager;
use crate::services::trend_list_service::{TrendListService, TrendListServiceImpl};
use chrono::NaiveDateTime;
use repos::Result;
use std::sync::Arc;

#[tokio::test]
async fn with_isu_empty() -> Result<()> {
    let mut repo = MockRepositoryManager::new();

    repo.isu_repository
        .expect_find_all_by_character()
        .returning(|_| Ok(Vec::new()));

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let trend = service.get_trend("test".to_string()).await?;
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

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let trend = service.get_trend("test".to_string()).await?;
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

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let trend = service.get_trend("test".to_string()).await?;
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

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let trend = service.get_trend("test".to_string()).await?;
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

    let service = TrendListServiceImpl::from_repo(Arc::new(repo));
    let trend = service.get_trend("test".to_string()).await?;
    assert_eq!(trend.character, "test".to_string());
    assert_eq!(trend.info.len(), 0);
    assert_eq!(trend.warning.len(), 0);
    assert_eq!(trend.critical.len(), 1);

    Ok(())
}
