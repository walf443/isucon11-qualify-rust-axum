use crate::database::DBConnectionPool;
use crate::models::isu::IsuUUID;
use crate::models::isu_condition::IsuCondition;
use crate::models::isu_condition::IsuConditionID;
use crate::repos::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IsuConditionRepository {
    async fn find_last_by_isu_id(&self, jia_isu_uuid: &IsuUUID) -> Result<Option<IsuCondition>>;
    async fn find_all_by_uuid(&self, jia_isu_uuid: &IsuUUID) -> Result<Vec<IsuCondition>>;
    async fn find_all_by_uuid_in_time(
        &self,
        jia_isu_uuid: &IsuUUID,
        start_time: Option<NaiveDateTime>,
        end_time: NaiveDateTime,
    ) -> Result<Vec<IsuCondition>>;
}

#[derive(Clone)]
pub struct IsuConditionRepositoryImpl {
    pub pool: DBConnectionPool,
}

impl IsuConditionRepositoryImpl {
    async fn find_all_by_uuid_in_time_range(
        &self,
        jia_isu_uuid: &IsuUUID,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<IsuCondition>> {
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
                WHERE `jia_isu_uuid` = ? AND timestamp < ? AND ? <= timestamp
                ORDER BY `timestamp` DESC"##,
            jia_isu_uuid.to_string(),
            end_time,
            start_time,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_all_by_uuid_until_time(
        &self,
        jia_isu_uuid: &IsuUUID,
        end_time: NaiveDateTime,
    ) -> Result<Vec<IsuCondition>> {
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
                WHERE `jia_isu_uuid` = ? AND timestamp < ?
                ORDER BY `timestamp` DESC"##,
            jia_isu_uuid.to_string(),
            end_time
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
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

    async fn find_all_by_uuid_in_time(
        &self,
        jia_isu_uuid: &IsuUUID,
        start_time: Option<NaiveDateTime>,
        end_time: NaiveDateTime,
    ) -> Result<Vec<IsuCondition>> {
        let result = match start_time {
            Some(start_time) => {
                self.find_all_by_uuid_in_time_range(jia_isu_uuid, start_time, end_time)
                    .await
            }
            None => {
                self.find_all_by_uuid_until_time(jia_isu_uuid, end_time)
                    .await
            }
        };
        result
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

    mod find_all_by_uuid_in_time_range {
        use crate::database::get_db_connection_for_test;
        use crate::models::isu::IsuUUID;
        use crate::repos;
        use crate::repos::isu_condition_repository::IsuConditionRepositoryImpl;
        use crate::test::Cleaner;
        use chrono::NaiveDateTime;

        #[tokio::test]
        async fn without_result() -> repos::Result<()> {
            let pool = get_db_connection_for_test().await;

            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("isu_condition").await?;

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let result = repo
                .find_all_by_uuid_in_time_range(
                    &IsuUUID::new("test".to_string()),
                    NaiveDateTime::from_timestamp(0, 0),
                    NaiveDateTime::from_timestamp(5, 0),
                )
                .await?;
            assert_eq!(result.len(), 0);

            cleaner.clean().await?;

            Ok(())
        }

        #[tokio::test]
        async fn with_result() -> repos::Result<()> {
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
            "1".to_string(),
            "2022-02-10T10:00:00".to_string(),
            true,
            "",
            "test2"
        ).execute(&pool).await?;

            let start_time =
                NaiveDateTime::parse_from_str("2022-02-10T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
            let end_time =
                NaiveDateTime::parse_from_str("2022-02-11T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let result = repo
                .find_all_by_uuid_in_time_range(
                    &IsuUUID::new("1".to_string()),
                    start_time,
                    end_time,
                )
                .await?;
            assert_eq!(result.len(), 1);

            cleaner.clean().await?;

            Ok(())
        }
    }

    mod find_all_by_uuid_until_time {
        use crate::database::get_db_connection_for_test;
        use crate::models::isu::IsuUUID;
        use crate::repos;
        use crate::repos::isu_condition_repository::IsuConditionRepositoryImpl;
        use crate::test::Cleaner;
        use chrono::NaiveDateTime;

        #[tokio::test]
        async fn without_result() -> repos::Result<()> {
            let pool = get_db_connection_for_test().await;

            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("isu_condition").await?;

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let result = repo
                .find_all_by_uuid_until_time(
                    &IsuUUID::new("test".to_string()),
                    NaiveDateTime::from_timestamp(5, 0),
                )
                .await?;
            assert_eq!(result.len(), 0);

            cleaner.clean().await?;

            Ok(())
        }

        #[tokio::test]
        async fn with_result() -> repos::Result<()> {
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
            "1".to_string(),
            "2022-02-10T10:00:00".to_string(),
            true,
            "",
            "test2"
        ).execute(&pool).await?;

            let end_time =
                NaiveDateTime::parse_from_str("2022-02-11T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

            let repo = IsuConditionRepositoryImpl { pool: pool };
            let result = repo
                .find_all_by_uuid_until_time(&IsuUUID::new("1".to_string()), end_time)
                .await?;
            assert_eq!(result.len(), 1);

            cleaner.clean().await?;

            Ok(())
        }
    }
}
