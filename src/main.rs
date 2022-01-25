use std::net::TcpListener;
use isucon11_qualify_rust_axum::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind port");

    let server = run(listener)?;

    server.await
}

