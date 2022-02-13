pub mod isu_association_config_repository;
pub mod isu_condition_repository;
pub mod isu_repository;
pub mod user_repository;

pub mod repository_manager;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("error for testing")]
    TestError(),
    #[error("failed to execute query")]
    SqlError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
