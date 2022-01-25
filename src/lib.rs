use std::future::Future;
use std::net::TcpListener;
use axum::{Json, Router, Server};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use crate::api::isu::{get_isu_graph, get_isu_icon, get_isu_id, get_isu_list, post_isu};

mod api;

pub fn run(listener: TcpListener) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
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

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
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
