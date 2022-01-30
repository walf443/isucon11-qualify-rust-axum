use async_trait::async_trait;
use sqlx::MySqlPool;

#[cfg(test)]
use crate::{DBConfig, get_db_connection};

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, jia_user_id: String) -> Result<(), anyhow::Error>;
}

pub struct UserRepositoryImpl {
    pool: MySqlPool
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, jia_user_id: String) -> Result<(), anyhow::Error> {
        sqlx::query!("INSERT IGNORE INTO user (`jia_user_id`) VALUES (?)", jia_user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_user_repository_insert() -> Result<(), sqlx::Error> {
    let dbconfg = DBConfig::default_for_test();
    let pool = get_db_connection(&dbconfg).await;
    let repo = UserRepositoryImpl { pool: pool.clone() };
    let result = repo.insert("1".to_string()).await;
    assert!(result.is_ok());
    let result = sqlx::query!("SELECT COUNT(*) as count FROM user WHERE jia_user_id = ?", "1").fetch_one(&pool).await?;

    assert_eq!(1, result.count);

    Ok(())
}

