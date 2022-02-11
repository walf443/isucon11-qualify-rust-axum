use isucon11_qualify_rust_axum::model::RepositoryManagerImpl;
use isucon11_qualify_rust_axum::{get_db_connection, run, DBConfig};
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind port");

    let dbconf = DBConfig::default();
    let pool = get_db_connection(dbconf).await;
    let repo_manager = Arc::new(RepositoryManagerImpl::new(pool));

    let server = run(listener, repo_manager)?;

    server.await
}
