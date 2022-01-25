use std::future::Future;
use std::net::TcpListener;
use axum::{Router, Server};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use crate::api::{get_index, get_me, get_trend, post_authentication, post_initialize, post_signout};
use crate::api::isu::{get_isu_graph, get_isu_icon, get_isu_id, get_isu_list, post_isu};
use crate::api::isu_condition::{get_isu_conditions, post_isu_condition};

mod api;

pub fn run(listener: TcpListener) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let app = Router::new()
        .route("/", get(get_index))
        .route("/initialize", post(post_initialize))
        .route("/api/signout", post(post_signout))
        .route("/api/user/me", get(get_me))
        .route("/api/isu", get(get_isu_list).post(post_isu))
        .route("/api/isu/:jia_isu_uuid", get(get_isu_id))
        .route("/api/isu/:jia_isu_uuid/icon", get(get_isu_icon))
        .route("/api/isu/:jia_isu_uuid/graph", get(get_isu_graph))
        .route("/api/isu/condition/:jia_isu_uuid", get(get_isu_conditions).post(post_isu_condition))
        .route("/api/trend", get(get_trend))
        .route("/api/auth", post(post_authentication));

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}

