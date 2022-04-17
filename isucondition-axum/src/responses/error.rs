use crate::responses::error::Error::{IsuNotFoundError, ModelError, UnauthorizedError};
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
    #[error("you are not signed in.")]
    UnauthorizedError(),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("error: {:?}", self);

        let code = match self {
            IsuNotFoundError() => StatusCode::NOT_FOUND,
            UnauthorizedError() => StatusCode::UNAUTHORIZED,
            ModelError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, format!("{:?}", self)).into_response()
    }
}
