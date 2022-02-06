use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use crate::{IntoResponse, MySqlPool, StatusCode};
use axum::{extract, Json};
use serde::Serialize;
use tower_cookies::Cookies;
use tracing::error;

#[derive(Serialize)]
struct GetMeResponse {
    jia_user_id: String,
}

pub async fn get_me(
    pool: extract::Extension<MySqlPool>,
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

    let repo = UserRepositoryImpl { pool: pool.0 };
    match repo.count_by_user_id(jia_user_id.to_string()).await {
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
