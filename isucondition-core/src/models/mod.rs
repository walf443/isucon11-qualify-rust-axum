pub mod isu;
pub mod isu_association_config;
pub mod isu_condition;
pub mod trend;
pub mod user;

use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error for testing")]
    TestError(),
    #[error("failed to parse URL")]
    HttpUrlParseError(),
    #[error("failed to parse URL")]
    UrlParseError(#[from] ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
