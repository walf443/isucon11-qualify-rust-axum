pub mod isu;
pub mod isu_condition;
pub mod user;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("error for testing")]
    TestError(),
}

pub type Result<T> = std::result::Result<T, Error>;
