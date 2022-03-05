use crate::database::DBConnectionPool;
use crate::models::isu_association_config::IsuAssociationConfigForm;
use crate::repos::Result;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IsuAssociationConfigRepository {
    async fn insert(&self, form: &IsuAssociationConfigForm) -> Result<()>;
    async fn get_jia_service_url(&self) -> Result<String>;
}

#[derive(Clone)]
pub struct IsuAssociationConfigRepositoryImpl {
    pub pool: DBConnectionPool,
}

#[async_trait]
impl IsuAssociationConfigRepository for IsuAssociationConfigRepositoryImpl {
    async fn insert(&self, form: &IsuAssociationConfigForm) -> Result<()> {
        sqlx::query!("INSERT INTO `isu_association_config` (`name`, `url`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `url` = VALUES(`url`)",
        form.name.to_string(), form.url.to_string())
            .fetch_all(&self.pool).await?;
        Ok(())
    }

    async fn get_jia_service_url(&self) -> Result<String> {
        let result = sqlx::query!(
            "SELECT url FROM isu_association_config WHERE name = ?",
            "jia_service_url"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.url)
    }
}
