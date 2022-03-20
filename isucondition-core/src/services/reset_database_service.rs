use crate::repos::Error::CommandExecutionError;
use crate::services;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait ResetDatabaseService {
    async fn run(&self) -> services::Result<()>;
}

#[derive(Clone)]
pub struct ResetDatabaseServiceImpl {}

impl ResetDatabaseServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ResetDatabaseService for ResetDatabaseServiceImpl {
    async fn run(&self) -> services::Result<()> {
        let status = tokio::process::Command::new("../sql/init.sh")
            .status()
            .await?;

        if !status.success() {
            return Err(CommandExecutionError());
        }

        Ok(())
    }
}
