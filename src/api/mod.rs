use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract, Json};
use serde::Serialize;
use sqlx::{Error, MySqlPool};
use tower_cookies::Cookies;
use tracing::error;

pub mod authorization;
pub mod initialize;
pub mod isu;
pub mod isu_condition;

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

    let repo = UserRepositoryImpl {
        pool: pool.0.clone(),
    };
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
pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

#[cfg(test)]
mod tests {
    use crate::model::cleaner::tests::Cleaner;
    use crate::test_helper;

    #[tokio::test]
    async fn test_get_index() -> Result<(), sqlx::Error> {
        let app = test_helper::spawn_app().await;
        let client = reqwest::Client::new();
        let res = client
            .get(app.url.join("/").unwrap())
            .send()
            .await
            .expect("Failed to request");

        assert!(res.status().is_success());

        Ok(())
    }
}
