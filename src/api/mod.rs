use axum::http::StatusCode;
use axum::{Json};
use axum::response::IntoResponse;

#[cfg(test)]
use crate::test_helper;

pub mod initialize;
pub mod isu;
pub mod isu_condition;

pub async fn post_signout() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_me() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

#[tokio::test]
async fn test_get_index() {
    let app = test_helper::spawn_app().await;
    let client = reqwest::Client::new();
    let res = client.get(app.url.join("/").unwrap()).send().await
        .expect("Failed to request");

    assert!(res.status().is_success());
}

pub async fn post_authentication() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
