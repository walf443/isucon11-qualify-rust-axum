use crate::database::DBConnectionPool;
use crate::models::isu::{Isu, IsuID, IsuUUID};
use crate::models::user::UserID;
use crate::repos::Result;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IsuRepository {
    async fn find_all_by_user_id(&self, jia_user_id: &UserID) -> Result<Vec<Isu>>;
}

#[derive(Clone)]
pub struct IsuRepositoryImpl {
    pub pool: DBConnectionPool,
}

#[async_trait]
impl IsuRepository for IsuRepositoryImpl {
    async fn find_all_by_user_id(&self, jia_user_id: &UserID) -> Result<Vec<Isu>> {
        let chairs = sqlx::query_as!(
            Isu,
            r##"SELECT
                id as `id:IsuID`,
                jia_user_id as `jia_user_id:UserID`,
                jia_isu_uuid as `jia_isu_uuid:IsuUUID`,
                name,
                `character`
            FROM isu WHERE jia_user_id = ?"##,
            jia_user_id.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(chairs)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::get_db_connection_for_test;
    use crate::models::user::UserID;
    use crate::repos::isu_repository::{IsuRepository, IsuRepositoryImpl};
    use crate::repos::Result;
    use crate::test::Cleaner;

    #[tokio::test]
    async fn test_find_all_by_user_id_empty() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_all_by_user_id(&UserID::new("1".to_string()))
            .await?;
        assert_eq!(result.len(), 0);

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_by_user_id_with_result() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        sqlx::query!(
            "INSERT INTO isu (jia_user_id, jia_isu_uuid, name) VALUES  (?,?,?), (?,?,?), (?,?,?)",
            "1".to_string(),
            "xxxx".to_string(),
            "test1".to_string(),
            "1".to_string(),
            "yyyy".to_string(),
            "test2".to_string(),
            "1".to_string(),
            "zzzz".to_string(),
            "test3".to_string(),
        )
        .execute(&pool)
        .await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_all_by_user_id(&UserID::new("1".to_string()))
            .await?;
        assert_eq!(result.len(), 3);

        cleaner.clean().await?;

        Ok(())
    }
}
