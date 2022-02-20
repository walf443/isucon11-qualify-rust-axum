use crate::responses::error::Error;
use crate::responses::trend_response::TrendResponse;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::repos::isu_repository::IsuRepository;
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::services::trend_service::TrendService;
use std::sync::Arc;

pub async fn get_trend<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
) -> Result<impl IntoResponse, Error> {
    let character_list = repo.isu_repository().find_character_group_by().await?;

    let mut trends: Vec<TrendResponse> = Vec::new();

    for character in character_list {
        if !character.is_none() {
            let character = character.unwrap();
            let trend_service = TrendService::new(repo.as_ref());
            let trend = trend_service.run(character).await?;

            trends.push(trend.into());
        }
    }

    Ok((StatusCode::OK, Json(trends)))
}
