use crate::model::isu_condition::IsuCondition;
use async_trait::async_trait;
use sqlx::{Error, MySqlPool};

#[async_trait]
pub trait IsuConditionRepository {
    async fn find_last_by_isu_id(
        &self,
        jia_isu_uuid: String,
    ) -> Result<Option<IsuCondition>, sqlx::Error>;
}

#[derive(Clone)]
pub struct IsuConditionRepositoryImpl {
    pub pool: MySqlPool,
}

#[async_trait]
impl IsuConditionRepository for IsuConditionRepositoryImpl {
    async fn find_last_by_isu_id(
        &self,
        jia_isu_uuid: String,
    ) -> Result<Option<IsuCondition>, Error> {
        let result = sqlx::query_as!(IsuCondition, "SELECT id, jia_isu_uuid, is_sitting as `is_sitting: bool`, `condition`, message, created_at, `timestamp` FROM `isu_condition` WHERE `jia_isu_uuid` = ? ORDER BY `timestamp` DESC LIMIT 1", jia_isu_uuid).fetch_optional(&self.pool).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::model::cleaner::tests::Cleaner;
    use crate::model::isu_condition::IsuCondition;
    use crate::model::isu_condition_repository::{
        IsuConditionRepository, IsuConditionRepositoryImpl,
    };
    use crate::{get_db_connection, DBConfig};

    #[tokio::test]
    async fn test_find_last_by_isu_id_with_empty() -> Result<(), sqlx::Error> {
        let dbconf = DBConfig::default_for_test();
        let pool = get_db_connection(&dbconf).await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu_condition").await?;

        let repo = IsuConditionRepositoryImpl { pool: pool };
        let condition = repo.find_last_by_isu_id("1".to_string()).await?;

        assert!(condition.is_none());

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_last_by_isu_id_with_result() -> Result<(), sqlx::Error> {
        let dbconf = DBConfig::default_for_test();
        let pool = get_db_connection(&dbconf).await;

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
        let condition = repo.find_last_by_isu_id("1".to_string()).await?;

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
