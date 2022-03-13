use async_redis_session::RedisSessionStore;
use isucondition_core::database::{get_db_connection, DBConfig, RedisConfig};
use isucondition_core::repos::repository_manager::RepositoryManagerImpl;
use isucondition_core::services::service_manager::ServiceManagerImpl;
use isucondition_http::run;
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind port");

    let dbconf = DBConfig::default();
    let pool = get_db_connection(dbconf).await;
    let repo_manager = Arc::new(RepositoryManagerImpl::new(pool));

    let redis_config = RedisConfig::default();
    let store = RedisSessionStore::new(redis_config.url).unwrap();
    let service_manager = Arc::new(ServiceManagerImpl::new(repo_manager.clone()));
    let server = run(listener, repo_manager, service_manager, store)?;

    server.await
}
