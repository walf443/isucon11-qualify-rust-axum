use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::models::isu::Isu;
use isucondition_core::models::isu_condition::IsuCondition;
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::services::isu_list_service::{IsuListService, IsuWithCondition};
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

impl From<IsuWithCondition> for IsuEntity {
    fn from((isu, latest_isu_condition): IsuWithCondition) -> Self {
        let condition = match latest_isu_condition {
            Some(condition) => Some(IsuConditionEntity {
                jia_isu_uuid: condition.jia_isu_uuid,
                isu_name: isu.name.clone(),
                timestamp: condition.timestamp.timestamp(),
                is_sitting: condition.is_sitting,
                condition: condition.condition,
                condition_level: "",
                message: condition.message,
            }),
            None => None,
        };
        Self {
            id: isu.id.to_string(),
            jia_isu_uuid: isu.jia_isu_uuid,
            name: isu.name,
            character: isu.character,
            latest_isu_condition: condition,
        }
    }
}

pub async fn get_isu_list<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let service = IsuListService::new(repo.as_ref().clone());
    let list = service.run("1".to_string()).await.map_err(|err| {
        error!("error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "error".to_string())
    })?;

    let list: Vec<IsuEntity> = list.into_iter().map(|isu| isu.into()).collect();

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
