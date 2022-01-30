use sqlx::MySqlPool;
use async_trait::async_trait;

#[async_trait]
pub trait IsuAssociationConfigRepository {
    async fn insert(&self, name: &str, url: &str) -> Result<(), anyhow::Error>;
}

pub struct IsuAssociationConfigRepositoryImpl {
    pub pool: MySqlPool
}

#[async_trait]
impl IsuAssociationConfigRepository for IsuAssociationConfigRepositoryImpl {
    async fn insert(&self , name: &str, url: &str) -> Result<(), anyhow::Error> {
        sqlx::query!("INSERT INTO `isu_association_config` (`name`, `url`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `url` = VALUES(`url`)",
        name, url)
            .fetch_all(&self.pool).await?;
        Ok(())
    }
}

