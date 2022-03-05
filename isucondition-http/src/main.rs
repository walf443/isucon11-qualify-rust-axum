use async_session::MemoryStore;
use isucondition_core::database::{get_db_connection, DBConfig};
use isucondition_core::repos::repository_manager::RepositoryManagerImpl;
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

    let store = MemoryStore::new();
    let server = run(listener, repo_manager, store)?;

    server.await
}
