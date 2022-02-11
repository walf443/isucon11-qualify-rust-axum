use crate::{run, RepositoryManagerImpl, get_db_connection_for_test};
use reqwest::Url;
use sqlx::MySqlPool;
use std::net::TcpListener;
use std::sync::Arc;

pub struct TestApp {
    pub address: String,
    pub url: Url,
    pub database: MySqlPool,
}

pub async fn spawn_app() -> TestApp {
    let pool = get_db_connection_for_test().await;
    let repo_manager = RepositoryManagerImpl::new(pool.clone());

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, Arc::new(repo_manager)).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{port}");
    TestApp {
        address: address.clone(),
        database: pool,
        url: Url::parse(&address).unwrap(),
    }
}
