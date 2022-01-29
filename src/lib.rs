use std::env;
use std::future::Future;
use std::net::TcpListener;
use std::time::Duration;
use axum::{AddExtensionLayer, Router, Server};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use sqlx::{Executor, MySql, Pool};
use crate::api::{get_index, get_me, get_trend, post_authentication, post_initialize, post_signout};
use crate::api::isu::{get_isu_graph, get_isu_icon, get_isu_id, get_isu_list, post_isu};
use crate::api::isu_condition::{get_isu_conditions, post_isu_condition};

mod api;

pub fn run(listener: TcpListener, dbpool: Pool<MySql>) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let app = Router::new()
        .layer(AddExtensionLayer::new(dbpool))
        .route("/", get(get_index))
        .route("/initialize", post(post_initialize))
        .route("/api/signout", post(post_signout))
        .route("/api/user/me", get(get_me))
        .route("/api/isu", get(get_isu_list).post(post_isu))
        .route("/api/isu/:jia_isu_uuid", get(get_isu_id))
        .route("/api/isu/:jia_isu_uuid/icon", get(get_isu_icon))
        .route("/api/isu/:jia_isu_uuid/graph", get(get_isu_graph))
        .route("/api/isu/condition/:jia_isu_uuid", get(get_isu_conditions).post(post_isu_condition))
        .route("/api/trend", get(get_trend))
        .route("/api/auth", post(post_authentication));

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}

pub async fn get_db_connection(config: &DBConfig) -> Pool<MySql> {
    let pool = sqlx::mysql::MySqlPoolOptions::new().connect_timeout(config.connect_timeout).after_connect(|conn| {
        Box::pin(async move {
            conn.execute("set time_zone = '+09:00'").await?;
            Ok(())
        })
    }).connect_with(sqlx::mysql::MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .database(&config.db_name)
        .username(&config.user)
        .password(&config.password)
    ).await.expect("can't connect db");
    pool
}

#[derive(Debug)]
pub struct DBConfig {
    host: String,
    port: u16,
    db_name: String,
    user: String,
    password: String,
    connect_timeout: Duration,
}

impl DBConfig {
    pub fn default_for_test() -> Self {
        let mut config = Self::default();
        config.db_name = env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned());
        config.connect_timeout = Duration::from_secs(1);
        config
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        let port= env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_owned()).parse().expect("port should be u16");
        Self {
            host: env::var("MYSQL_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            port: port,
            user: env::var("MYSQL_USER").unwrap_or_else(|_| "isucon".to_owned()),
            db_name: env::var("MYSQL_DBNAME").unwrap_or_else(|_| "isucondition".to_owned()),
            password: env::var("MYSQL_PASS").unwrap_or_else(|_| "isucon".to_owned()),
            connect_timeout: Duration::from_secs(30),
        }
    }
}