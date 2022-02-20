use crate::database::DBConnectionPool;
use crate::models::isu::{Isu, IsuID, IsuUUID};
use crate::models::user::UserID;
use crate::repos;
use crate::repos::{Error, Result};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IsuRepository {
    async fn find_all_by_user_id(&self, jia_user_id: &UserID) -> Result<Vec<Isu>>;
    async fn find_image_by_uuid_and_user_id(
        &self,
        jia_uuid: &IsuUUID,
        jia_user_id: &UserID,
    ) -> Result<Option<Vec<u8>>>;
    async fn find_by_uuid_and_user_id(
        &self,
        jia_uuid: &IsuUUID,
        jia_user_id: &UserID,
    ) -> Result<Option<Isu>>;
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

    async fn find_image_by_uuid_and_user_id(
        &self,
        jia_uuid: &IsuUUID,
        jia_user_id: &UserID,
    ) -> Result<Option<Vec<u8>>> {
        let result = sqlx::query!(
            "SELECT image FROM isu WHERE jia_isu_uuid = ? AND jia_user_id = ?",
            jia_uuid.to_string(),
            jia_user_id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => Ok(record.image),
            Err(err) => match err {
                sqlx::error::Error::RowNotFound => Ok(None),
                _ => Err(Error::SqlError(err)),
            },
        }
    }

    async fn find_by_uuid_and_user_id(
        &self,
        jia_uuid: &IsuUUID,
        jia_user_id: &UserID,
    ) -> Result<Option<Isu>> {
        let result = sqlx::query_as!(
            Isu,
            r##"SELECT
                id as `id:IsuID`,
                jia_user_id as `jia_user_id:UserID`,
                jia_isu_uuid as `jia_isu_uuid:IsuUUID`,
                name,
                `character`
            FROM isu WHERE jia_isu_uuid = ? AND jia_user_id = ?"##,
            jia_uuid.to_string(),
            jia_user_id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => Ok(Some(record)),
            Err(err) => match err {
                sqlx::error::Error::RowNotFound => Ok(None),
                _ => Err(repos::Error::SqlError(err)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::get_db_connection_for_test;
    use crate::models::isu::IsuUUID;
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

    #[tokio::test]
    async fn test_find_image_by_uuid_and_user_id_with_empty() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_image_by_uuid_and_user_id(
                &IsuUUID::new("test".to_string()),
                &UserID::new("1".to_string()),
            )
            .await?;

        assert!(result.is_none());

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_image_by_uuid_and_user_id_with_result() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        sqlx::query!(
            "INSERT INTO isu (jia_user_id, jia_isu_uuid, name, image) VALUES  (?,?,?,?)",
            "1".to_string(),
            "test".to_string(),
            "test1".to_string(),
            "a".to_string(),
        )
        .execute(&pool)
        .await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_image_by_uuid_and_user_id(
                &IsuUUID::new("test".to_string()),
                &UserID::new("1".to_string()),
            )
            .await?;

        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![b'a']);

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_uuid_and_user_id_with_empty() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_by_uuid_and_user_id(
                &IsuUUID::new("test".to_string()),
                &UserID::new("1".to_string()),
            )
            .await?;

        assert!(result.is_none());

        cleaner.clean().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_uuid_and_user_id_with_result() -> Result<()> {
        let pool = get_db_connection_for_test().await;

        let mut cleaner = Cleaner::new(pool.clone());
        cleaner.prepare_table("isu").await?;

        sqlx::query!(
            "INSERT INTO isu (jia_user_id, jia_isu_uuid, name) VALUES  (?,?,?)",
            "1".to_string(),
            "test".to_string(),
            "test1".to_string(),
        )
        .execute(&pool)
        .await?;

        let repo = IsuRepositoryImpl { pool: pool };
        let result = repo
            .find_by_uuid_and_user_id(
                &IsuUUID::new("test".to_string()),
                &UserID::new("1".to_string()),
            )
            .await?;

        assert!(result.is_some());

        cleaner.clean().await?;

        Ok(())
    }
}
