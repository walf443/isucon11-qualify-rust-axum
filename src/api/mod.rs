use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract, Json};
use sqlx::MySqlPool;
use tower_cookies::{Cookie, Cookies};
use tracing::log;

pub mod initialize;
pub mod isu;
pub mod isu_condition;

pub async fn post_signout() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_me() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn post_authentication(
    pool: extract::Extension<MySqlPool>,
    cookies: Cookies,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let jia_user_id = "1";
    let user_repo = UserRepositoryImpl { pool: pool.0 };
    user_repo
        .insert(jia_user_id.to_string())
        .await
        .map_err(|e| {
            log::error!("user insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "user insert failed".to_string(),
            )
        })?;
    cookies.add(Cookie::new("jia_user_id", jia_user_id));

    Ok((StatusCode::OK, Json(vec!["Hello, world"])))
}

#[cfg(test)]
mod tests {
    use crate::model::cleaner::tests::Cleaner;
    use crate::test_helper;

    #[tokio::test]
    async fn test_get_index() -> Result<(), sqlx::Error> {
        let app = test_helper::spawn_app().await;
        let mut cleaner = Cleaner::new(app.database.clone());

        cleaner.prepare_table("user").await?;

        let client = reqwest::Client::new();
        let res = client
            .get(app.url.join("/").unwrap())
            .send()
            .await
            .expect("Failed to request");

        assert!(res.status().is_success());
        cleaner.clean().await?;

        Ok(())
    }
}
