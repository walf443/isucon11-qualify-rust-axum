use crate::responses::error::Error::{IsuNotFoundError, ModelError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use isucondition_core::{models, repos};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]

pub enum Error {
    #[error("failed to get data")]
    ReposError(#[from] repos::Error),
    #[error("failed to parse user input")]
    ModelError(#[from] models::Error),
    #[error("not found isu")]
    IsuNotFoundError(),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("error: {:?}", self);

        let code = match self {
            IsuNotFoundError() => StatusCode::NOT_FOUND,
            ModelError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, code.to_string()).into_response()
    }
}
