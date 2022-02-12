use crate::database::DBConnectionPool;
use crate::models::isu_condition::IsuCondition;
use crate::repos::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IsuConditionRepository {
    async fn find_last_by_isu_id(&self, jia_isu_uuid: &str) -> Result<Option<IsuCondition>>;
}

#[derive(Clone)]
pub struct IsuConditionRepositoryImpl {
    pub pool: DBConnectionPool,
}

#[async_trait]
impl IsuConditionRepository for IsuConditionRepositoryImpl {
    async fn find_last_by_isu_id(&self, jia_isu_uuid: &str) -> Result<Option<IsuCondition>> {
        let result = sqlx::query_as!(IsuCondition, "SELECT id, jia_isu_uuid, is_sitting as `is_sitting: bool`, `condition`, message, created_at, `timestamp` FROM `isu_condition` WHERE `jia_isu_uuid` = ? ORDER BY `timestamp` DESC LIMIT 1", jia_isu_uuid).fetch_optional(&self.pool).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::database::get_db_connection_for_test;
    use crate::models::isu_condition::IsuCondition;
    use crate::repos::isu_condition_repository::{
        IsuConditionRepository, IsuConditionRepositoryImpl,
    };
    use crate::repos::Result;
    use crate::test::Cleaner;

    #[tokio::test]
    async fn test_find_last_by_isu_id_with_empty() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu_condition").await?;

        let repo = IsuConditionRepositoryImpl { pool: pool };
        let condition = repo.find_last_by_isu_id("1").await?;

        assert!(condition.is_none());

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_last_by_isu_id_with_result() -> Result<()> {
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
        let condition = repo.find_last_by_isu_id("1").await?;

        assert!(condition.is_some());
        let condition = condition.unwrap();
        assert_eq!(
            IsuCondition {
                id: condition.id,
                timestamp: condition.timestamp.clone(),
                created_at: condition.created_at.clone(),
                jia_isu_uuid: "1".to_string(),
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
