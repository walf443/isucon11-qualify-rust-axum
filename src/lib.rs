use crate::api::authorization::{post_authentication, post_signout};
use crate::api::initialize::post_initialize;
use crate::api::isu::{get_isu_graph, get_isu_icon, get_isu_id, get_isu_list, post_isu};
use crate::api::isu_condition::{get_isu_conditions, post_isu_condition};
use crate::api::{get_index, get_trend};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{AddExtensionLayer, Router, Server};
use isucondition_core::repos::repository_manager::{RepositoryManager, RepositoryManagerImpl};
use isucondition_http::routes::user_routes::get_me;
use std::future::Future;
use std::net::TcpListener;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

#[cfg(test)]
mod test_helper;

mod api;

type Repo = RepositoryManagerImpl;

pub fn run<R: 'static + RepositoryManager>(
    listener: TcpListener,
    repo_manager: Arc<R>,
) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let repo_manager_layer = AddExtensionLayer::new(repo_manager);

    let app = Router::new()
        .route("/", get(get_index))
        .route("/initialize", post(post_initialize::<Repo>))
        .route("/api/signout", post(post_signout))
        .route("/api/user/me", get(get_me::<Repo>))
        .route("/api/isu", get(get_isu_list::<Repo>).post(post_isu))
        .route("/api/isu/:jia_isu_uuid", get(get_isu_id))
        .route("/api/isu/:jia_isu_uuid/icon", get(get_isu_icon))
        .route("/api/isu/:jia_isu_uuid/graph", get(get_isu_graph))
        .route(
            "/api/isu/condition/:jia_isu_uuid",
            get(get_isu_conditions).post(post_isu_condition),
        )
        .route("/api/trend", get(get_trend))
        .route("/api/auth", post(post_authentication::<Repo>))
        .layer(repo_manager_layer)
        .layer(CookieManagerLayer::new());

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
