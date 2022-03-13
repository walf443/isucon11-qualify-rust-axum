use crate::responses::error::Error;
use crate::responses::trend_response::TrendResponse;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::services::service_manager::ServiceManager;
use isucondition_core::services::trend_list_service::TrendListService;
use std::sync::Arc;

pub async fn get_trend<Service: ServiceManager>(
    Extension(service): Extension<Arc<Service>>,
) -> Result<impl IntoResponse, Error> {
    let trends = service.trend_list_service().run().await?;
    let trends: Vec<TrendResponse> = trends.into_iter().map(|t| t.into()).collect();

    Ok((StatusCode::OK, Json(trends)))
}
