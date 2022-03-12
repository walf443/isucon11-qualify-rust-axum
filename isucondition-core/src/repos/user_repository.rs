use crate::database::DBConnectionPool;
use crate::models::user::UserID;
use crate::repos::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UserRepository {
    async fn insert(&self, jia_user_id: String) -> Result<()>;
    async fn count_by_user_id(&self, jia_user_id: &UserID) -> Result<i64>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pub pool: DBConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, jia_user_id: String) -> Result<()> {
        sqlx::query!(
            "INSERT IGNORE INTO user (`jia_user_id`) VALUES (?)",
            jia_user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(())
    }

    async fn count_by_user_id(&self, jia_user_id: &UserID) -> Result<i64> {
        let result = sqlx::query!(
            "SElECT COUNT(*) as count FROM user WHERE jia_user_id = ?",
            jia_user_id.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count)
    }
}

#[cfg(test)]
mod tests {
    mod insert {
        use crate::database::get_db_connection_for_test;
        use crate::repos::user_repository::{UserRepository, UserRepositoryImpl};
        use crate::repos::Result;
        use crate::test::Cleaner;

        #[tokio::test]
        async fn test_user_repository_insert() -> Result<()> {
            let pool = get_db_connection_for_test().await;
            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("user").await?;

            let repo = UserRepositoryImpl { pool: pool.clone() };
            let result = repo.insert("1".to_string()).await?;
            let result = sqlx::query!(
                "SELECT COUNT(*) as count FROM user WHERE jia_user_id = ?",
                "1"
            )
            .fetch_one(&pool)
            .await?;

            assert_eq!(1, result.count);

            cleaner.clean().await?;

            Ok(())
        }
    }

    mod count_by_user_id {
        use crate::database::get_db_connection_for_test;
        use crate::models::user::UserID;
        use crate::repos::user_repository::{UserRepository, UserRepositoryImpl};
        use crate::repos::Result;
        use crate::test::Cleaner;

        #[tokio::test]
        async fn with_result() -> Result<()> {
            let pool = get_db_connection_for_test().await;
            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("user").await?;

            sqlx::query!("INSERT INTO user (jia_user_id) VALUES (1)")
                .execute(&pool)
                .await?;

            let repo = UserRepositoryImpl { pool: pool };

            let result = repo.count_by_user_id(&UserID::new("1".to_string())).await?;
            assert_eq!(result, 1);

            cleaner.clean().await?;

            Ok(())
        }

        #[tokio::test]
        async fn without_result() -> Result<()> {
            let pool = get_db_connection_for_test().await;
            let mut cleaner = Cleaner::new(pool.clone());
            cleaner.prepare_table("user").await?;
            let repo = UserRepositoryImpl { pool: pool };

            let result = repo.count_by_user_id(&UserID::new("1".to_string())).await?;
            assert_eq!(0, result);

            cleaner.clean().await?;

            Ok(())
        }
    }
}
