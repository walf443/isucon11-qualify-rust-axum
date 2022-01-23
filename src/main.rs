use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{get,post};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(get_index))
        .route("/initialize", post(post_initialize))
        .route("/api/signout", post(post_signout))
        .route("/api/user/me", get(get_me))
        .route("/api/isu", get(get_isu_list))
        .route("/api/isu", post(post_isu))
        .route("/api/isu/:jia_isu_uuid", get(get_isu_id))
        .route("/api/isu/:jia_isu_uuid/icon", get(get_isu_icon))
        .route("/api/isu/:jia_isu_uuid/graph", get(get_isu_graph))
        .route("/api/isu/condition/:jia_isu_uuid", get(get_isu_conditions))
        .route("/api/isu/condition/:jia_isu_uuid", post(post_isu_condition))
        .route("/api/trend", get(get_trend))
        .route("/api/auth", post(post_authentication));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn post_initialize() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn post_signout() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_me() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
async fn get_isu_list() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn post_isu() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_isu_id(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_isu_icon(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_isu_graph(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_isu_conditions(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn post_isu_condition(Path(_jia_isu_uuid): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}

async fn post_authentication() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!("Hello, world")))
}
