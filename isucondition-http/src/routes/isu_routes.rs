use crate::responses::error::Error;
use crate::responses::error::Error::IsuNotFoundError;
use crate::responses::isu_response::IsuResponse;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::models::isu::IsuUUID;
use isucondition_core::models::user::UserID;
use isucondition_core::repos::isu_repository::IsuRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::services::isu_list_service::IsuListService;
use std::sync::Arc;
use tracing::error;

pub async fn get_isu_list<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
) -> Result<impl IntoResponse, Error> {
    let service = IsuListService::new(repo.as_ref());
    let list = service.run(UserID::new("1".to_string())).await?;

    let list: Vec<IsuResponse> = list.into_iter().map(|isu| isu.into()).collect();

    Ok((StatusCode::OK, Json(list)))
}

pub async fn post_isu() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_isu_id(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_isu_icon<Repo: RepositoryManager>(
    Path(jia_isu_uuid): Path<String>,
    Extension(repo): Extension<Arc<Repo>>,
) -> Result<impl IntoResponse, Error> {
    let uuid = IsuUUID::parse(jia_isu_uuid)?;

    let image = repo
        .isu_repository()
        .find_image_by_uuid_and_user_id(&uuid, &UserID::new("1".to_string()))
        .await?;

    match image {
        Some(image) => Ok(image),
        None => Err(IsuNotFoundError()),
    }
}

pub async fn get_isu_graph(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
