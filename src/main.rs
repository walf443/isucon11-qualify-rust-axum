use isucon11_qualify_rust_axum::{get_db_connection, run, DBConfig};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind port");

    let dbconf = DBConfig::default();
    let pool = get_db_connection(&dbconf).await;

    let server = run(listener, pool)?;

    server.await
}
