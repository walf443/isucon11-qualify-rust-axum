use sqlx::mysql::MySqlConnectOptions;
use sqlx::{Executor, MySqlPool};
use std::env;
use std::time::Duration;

pub type DBConnectionPool = MySqlPool;

#[derive(Debug)]
pub struct DBConfig {
    host: String,
    port: u16,
    db_name: String,
    user: String,
    password: String,
    acquire_timeout: Duration,
}

impl DBConfig {
    pub fn default_for_test() -> Self {
        let mut config = Self::default();
        config.db_name =
            env::var("MYSQL_DBNAME_TEST").unwrap_or_else(|_| "isucondition_test".to_owned());
        config.acquire_timeout = Duration::from_secs(1);
        config
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        let port = env::var("MYSQL_PORT")
            .unwrap_or_else(|_| "3306".to_owned())
            .parse()
            .expect("port should be u16");
        Self {
            host: env::var("MYSQL_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            port: port,
            user: env::var("MYSQL_USER").unwrap_or_else(|_| "isucon".to_owned()),
            db_name: env::var("MYSQL_DBNAME").unwrap_or_else(|_| "isucondition".to_owned()),
            password: env::var("MYSQL_PASS").unwrap_or_else(|_| "isucon".to_owned()),
            acquire_timeout: Duration::from_secs(30),
        }
    }
}

pub async fn get_db_connection(config: DBConfig) -> DBConnectionPool {
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .acquire_timeout(config.acquire_timeout)
        .after_connect(|conn, _metadata| {
            Box::pin(async move {
                conn.execute("set time_zone = '+09:00'").await?;
                Ok(())
            })
        })
        .connect_with(
            sqlx::mysql::MySqlConnectOptions::new()
                .host(&config.host)
                .port(config.port)
                .database(&config.db_name)
                .username(&config.user)
                .password(&config.password),
        )
        .await
        .expect("can't connect db");
    pool
}

pub async fn get_db_connection_for_test() -> DBConnectionPool {
    get_db_connection(DBConfig::default_for_test()).await
}

pub struct RedisConfig {
    pub url: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
        }
    }
}
