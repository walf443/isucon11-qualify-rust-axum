use crate::{IntoResponse, StatusCode};
use axum::extract::{Extension, Path};
use axum::Json;
use isucondition_core::repos::isu_condition_repository::IsuConditionRepository;
use isucondition_core::repos::isu_repository::IsuRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use serde::Serialize;
use std::sync::Arc;
use tracing::error;

#[derive(Serialize)]
struct IsuEntity {
    id: String,
    jia_isu_uuid: String,
    name: String,
    character: Option<String>,
    latest_isu_condition: Option<IsuConditionEntity>,
}

#[derive(Serialize)]
struct IsuConditionEntity {
    jia_isu_uuid: String,
    isu_name: String,
    timestamp: i64,
    is_sitting: bool,
    condition: String,
    condition_level: &'static str,
    message: String,
}

pub async fn get_isu_list<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let chairs = repo
        .isu_repository()
        .find_all_by_user_id("1".to_string())
        .await
        .map_err(|err| {
            error!("error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "can't find_all_by_user_id isu".to_string(),
            )
        })?;

    let mut list: Vec<IsuEntity> = Vec::new();
    for chair in chairs {
        let last_isu_condition = repo
            .isu_condition_repository()
            .find_last_by_isu_id(&chair.jia_isu_uuid)
            .await
            .map_err(|err| {
                error!("error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "can't find_last_by_isu_id isu_condition".to_string(),
                )
            })?;

        let last_condition_entity = match last_isu_condition {
            Some(condition) => Some(IsuConditionEntity {
                jia_isu_uuid: condition.jia_isu_uuid.to_string(),
                isu_name: chair.name.clone(),
                timestamp: condition.timestamp.timestamp(),
                is_sitting: condition.is_sitting,
                condition: condition.condition,
                condition_level: "",
                message: condition.message,
            }),
            None => None,
        };

        let entity = IsuEntity {
            id: chair.id.to_string(),
            jia_isu_uuid: chair.jia_isu_uuid.clone(),
            name: chair.name.clone(),
            character: chair.character.clone(),
            latest_isu_condition: last_condition_entity,
        };
        list.push(entity)
    }

    Ok((StatusCode::OK, Json(list)))
}

pub async fn post_isu() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_isu_id(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_isu_icon(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_isu_graph(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
