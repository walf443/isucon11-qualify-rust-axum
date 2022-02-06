use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json};
use sqlx::{MySqlPool};

pub mod authorization;
pub mod initialize;
pub mod isu;
pub mod isu_condition;
pub mod user;

pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

#[cfg(test)]
mod tests {
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
