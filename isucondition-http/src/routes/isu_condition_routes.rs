use crate::responses::error::Error;
use crate::responses::error::Error::IsuNotFoundError;
use crate::responses::isu_condition_response::IsuConditionResponse;
use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDateTime;
use isucondition_core::models::isu::IsuUUID;
use isucondition_core::models::user::UserID;
use isucondition_core::repos::isu_condition_repository::IsuConditionRepository;
use isucondition_core::repos::isu_repository::IsuRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetIsuConditionQuery {
    start_time: Option<String>,
    end_time: Option<String>,
    condition_level: Option<String>,
}

pub async fn get_isu_conditions<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
    query: Query<GetIsuConditionQuery>,
    Path(jia_isu_uuid): Path<String>,
) -> Result<impl IntoResponse, Error> {
    let uuid = IsuUUID::parse(jia_isu_uuid)?;
    let start_time = Some(NaiveDateTime::from_timestamp(0, 0));
    let end_time = NaiveDateTime::from_timestamp(0, 0);

    let isu = repo
        .isu_repository()
        .find_by_uuid_and_user_id(&uuid, &UserID::new("1".to_string()))
        .await?;
    if isu.is_none() {
        return Err(IsuNotFoundError());
    }
    let isu = isu.unwrap();

    let result = repo
        .isu_condition_repository()
        .find_all_by_uuid_in_time(&isu.jia_isu_uuid, start_time, end_time)
        .await?;
    let result: Vec<IsuConditionResponse> = result
        .into_iter()
        .map(|cond| (isu.clone(), cond).into())
        .collect();
    Ok((StatusCode::OK, Json(result)))
}

pub async fn post_isu_condition(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
