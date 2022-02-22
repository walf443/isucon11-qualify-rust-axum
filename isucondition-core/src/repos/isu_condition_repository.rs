use crate::database::DBConnectionPool;
use crate::models::isu::IsuUUID;
use crate::models::isu_condition::IsuCondition;
use crate::models::isu_condition::IsuConditionID;
use crate::repos::Result;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IsuConditionRepository {
    async fn find_last_by_isu_id(&self, jia_isu_uuid: &IsuUUID) -> Result<Option<IsuCondition>>;
    async fn find_all_by_uuid(&self, jia_isu_uuid: &IsuUUID) -> Result<Vec<IsuCondition>>;
}

#[derive(Clone)]
pub struct IsuConditionRepositoryImpl {
    pub pool: DBConnectionPool,
}

#[async_trait]
impl IsuConditionRepository for IsuConditionRepositoryImpl {
    async fn find_last_by_isu_id(&self, jia_isu_uuid: &IsuUUID) -> Result<Option<IsuCondition>> {
        let result = sqlx::query_as!(
            IsuCondition,
            r##"SELECT
                    id AS `id:IsuConditionID`,
                    jia_isu_uuid AS `jia_isu_uuid:IsuUUID`,
                    is_sitting as `is_sitting: bool`,
                    `condition`,
                    message,
                    created_at,
                    `timestamp`
                FROM `isu_condition`
                WHERE `jia_isu_uuid` = ?
                ORDER BY `timestamp` DESC LIMIT 1"##,
            jia_isu_uuid.to_string(),
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn find_all_by_uuid(&self, jia_isu_uuid: &IsuUUID) -> Result<Vec<IsuCondition>> {
        let result = sqlx::query_as!(
            IsuCondition,
            r##"SELECT
                    id AS `id:IsuConditionID`,
                    jia_isu_uuid AS `jia_isu_uuid:IsuUUID`,
                    is_sitting as `is_sitting: bool`,
                    `condition`,
                    message,
                    created_at,
                    `timestamp`
                FROM `isu_condition`
                WHERE `jia_isu_uuid` = ?
                ORDER BY `timestamp` DESC"##,
            jia_isu_uuid.to_string(),
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {

    mod find_last_by_isu_id {
        use crate::database::get_db_connection_for_test;
        use crate::models::isu::IsuUUID;
        use crate::models::isu_condition::IsuCondition;
        use crate::repos::isu_condition_repository::{
            IsuConditionRepository, IsuConditionRepositoryImpl,
        };
        use crate::repos::Result;
        use crate::test::Cleaner;

        #[tokio::test]
        async fn with_empty() -> Result<()> {
            let pool = get_db_connection_for_test().await;

            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("isu_condition").await?;

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let condition = repo
                .find_last_by_isu_id(&IsuUUID::new("1".to_string()))
                .await?;

            assert!(condition.is_none());

            cleaner.clean().await?;

            Ok(())
        }

        #[tokio::test]
        async fn with_result() -> Result<()> {
            let pool = get_db_connection_for_test().await;

            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("isu_condition").await?;

            sqlx::query!(
            "INSERT INTO isu_condition (jia_isu_uuid, timestamp, is_sitting, `condition`, message) VALUES  (?, ?, ?, ?, ?), (?, ?, ?, ?, ?)",
            "1".to_string(),
            "2022-02-11T10:00:00".to_string(),
            true,
            "",
            "test",
            "2".to_string(),
            "2022-02-10T10:00:00".to_string(),
            true,
            "",
            "test2"
        ).execute(&pool).await?;

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let condition = repo
                .find_last_by_isu_id(&IsuUUID::new("1".to_string()))
                .await?;

            assert!(condition.is_some());
            let condition = condition.unwrap();
            assert_eq!(
                IsuCondition {
                    id: condition.id.clone(),
                    timestamp: condition.timestamp.clone(),
                    created_at: condition.created_at.clone(),
                    jia_isu_uuid: IsuUUID::new("1".to_string()),
                    is_sitting: true,
                    condition: "".to_string(),
                    message: "test".to_string(),
                },
                condition
            );

            cleaner.clean().await?;

            Ok(())
        }
    }
}
