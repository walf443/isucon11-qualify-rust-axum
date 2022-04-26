use crate::repos;
use crate::repos::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
pub trait Transaction<'t> {
    async fn commit(self) -> Result<(), repos::Error>;
}

pub struct TransactionImpl<'t> {
    pub tx: sqlx::Transaction<'t, MySql>,
}

impl<'t> TransactionImpl<'t> {
    pub fn new(tx: sqlx::Transaction<'t, MySql>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'t> Transaction<'t> for TransactionImpl<'t> {
    async fn commit(self) -> Result<(), Error> {
        let result = self.tx.commit().await?;
        Ok(result)
    }
}
