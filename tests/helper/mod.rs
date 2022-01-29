use std::net::TcpListener;
use sqlx::Pool;
use sqlx::MySql;
use isucon11_qualify_rust_axum::{DBConfig, get_db_connection, run};

pub struct TestApp {
    pub address: String,
    pub database: Pool<MySql>,
}

pub async fn spawn_app() -> TestApp {
    let dbconf = DBConfig::default_for_test();
    let pool = get_db_connection(&dbconf).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, pool.clone()).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{port}");
    TestApp { address: address, database: pool }
}
