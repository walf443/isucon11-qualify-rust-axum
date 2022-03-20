#![forbid(unsafe_code)]

extern crate core;

mod requests;
mod responses;
mod routes;

use crate::routes::authorization_routes::{post_authentication, post_signout};
use crate::routes::get_index;
use crate::routes::initialize_routes::post_initialize;
use crate::routes::isu_condition_routes::{get_isu_conditions, post_isu_condition};
use crate::routes::isu_routes::{get_isu_graph, get_isu_icon, get_isu_id, get_isu_list, post_isu};
use crate::routes::trend_routes::get_trends;
use crate::routes::user_routes::get_me;
use async_session::SessionStore;
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{Router, Server};
use isucondition_core::repos::repository_manager::{RepositoryManager, RepositoryManagerImpl};
use isucondition_core::services::service_manager::{ServiceManager, ServiceManagerImpl};
use std::future::Future;
use std::net::TcpListener;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

#[cfg(test)]
mod test_helper;

type Repo = RepositoryManagerImpl;
type Service = ServiceManagerImpl<Repo>;

pub fn run<R: 'static + RepositoryManager, S: 'static + ServiceManager, Store: SessionStore>(
    listener: TcpListener,
    repo_manager: Arc<R>,
    service_manager: Arc<S>,
    session_store: Store,
) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let repo_manager_layer = Extension(repo_manager);
    let service_manager_layer = Extension(service_manager);

    let app = Router::new()
        .route("/", get(get_index))
        .route("/initialize", post(post_initialize::<Repo, Service>))
        .route("/api/signout", post(post_signout))
        .route("/api/user/me", get(get_me::<Repo>))
        .route("/api/isu", get(get_isu_list::<Service>).post(post_isu))
        .route("/api/isu/:jia_isu_uuid", get(get_isu_id::<Repo>))
        .route("/api/isu/:jia_isu_uuid/icon", get(get_isu_icon::<Repo>))
        .route("/api/isu/:jia_isu_uuid/graph", get(get_isu_graph))
        .route(
            "/api/isu/condition/:jia_isu_uuid",
            get(get_isu_conditions::<Repo>).post(post_isu_condition),
        )
        .route("/api/trend", get(get_trends::<Service>))
        .route("/api/auth", post(post_authentication::<Repo>))
        .layer(repo_manager_layer)
        .layer(service_manager_layer)
        .layer(Extension(session_store))
        .layer(CookieManagerLayer::new());

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
