use crate::model::user_repository::UserRepository;
use crate::{IntoResponse, RepositoryManager, StatusCode};
use axum::extract::Extension;
use axum::Json;
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
        .count_by_user_id(jia_user_id.to_string())
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
