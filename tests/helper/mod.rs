use std::net::TcpListener;
use isucon11_qualify_rust_axum::run;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{port}");
    TestApp { address: address }
}