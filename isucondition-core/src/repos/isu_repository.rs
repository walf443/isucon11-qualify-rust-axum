use crate::database::DBConnectionPool;
use crate::models::isu::{Isu, IsuID, IsuUUID};
use crate::models::user::UserID;
use crate::repos;
use crate::repos::{Error, Result};
use async_trait::async_trait;

#[cfg(test)]
mod tests;

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
    async fn find_character_group_by(&self) -> Result<Vec<Option<String>>>;
    async fn find_all_by_character(&self, character: &String) -> Result<Vec<Isu>>;
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

    async fn find_character_group_by(&self) -> Result<Vec<Option<String>>> {
        let result = sqlx::query!("SELECT `character` FROM isu GROUP BY `character`")
            .fetch_all(&self.pool)
            .await?;
        let characters = result.into_iter().map(|record| record.character).collect();

        Ok(characters)
    }

    async fn find_all_by_character(&self, character: &String) -> Result<Vec<Isu>> {
        let result = sqlx::query_as!(
            Isu,
            r##"SELECT
                id as `id:IsuID`,
                jia_user_id as `jia_user_id:UserID`,
                jia_isu_uuid as `jia_isu_uuid:IsuUUID`,
                name,
                `character`
            FROM isu WHERE `character` = ?"##,
            character,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}
