use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use isucondition_core::models::user::UserID;
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::repos::user_repository::UserRepository;
use serde::Serialize;
use std::sync::Arc;
use tower_cookies::Cookies;
use tracing::error;

#[derive(Serialize)]
struct GetMeResponse {
    jia_user_id: String,
}

pub async fn get_me<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let cookie = cookies.get("jia_user_id");
    if cookie.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "you are not signed in".to_string(),
        ));
    }
    let cookie = cookie.unwrap();
    let jia_user_id = cookie.value();

    match repo
        .user_repository()
        .count_by_user_id(&UserID::new(jia_user_id.to_string()))
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(GetMeResponse {
                jia_user_id: jia_user_id.to_string(),
            }),
        )),
        Err(e) => {
            error!("error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ))
        }
    }
}
