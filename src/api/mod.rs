use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use axum::extract::TypedHeader;
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract, headers, Json};
use sqlx::MySqlPool;
use tower_cookies::{Cookie, Cookies};
use tracing::log;

const JIA_JWT_SIGNING_KEY_PATH: &str = "ec256-public.pem";

lazy_static::lazy_static! {
    static ref JIA_JWT_SIGNING_KEY_PEM: Vec<u8> = std::fs::read(JIA_JWT_SIGNING_KEY_PATH).expect("failed to read JIA JWT signing key file");
    static ref JIA_JWT_SIGNING_KEY: jsonwebtoken::DecodingKey = jsonwebtoken::DecodingKey::from_ec_pem(&JIA_JWT_SIGNING_KEY_PEM).expect("failed to parse JIA JWT signing key");
}

pub mod initialize;
pub mod isu;
pub mod isu_condition;

pub async fn post_signout() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_me() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}
pub async fn get_trend() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

pub async fn get_index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec!["Hello, world"]))
}

#[derive(Debug, serde::Deserialize)]
struct Claims {
    jia_user_id: String,
}

pub async fn post_authentication(
    pool: extract::Extension<MySqlPool>,
    TypedHeader(authorization): axum::extract::TypedHeader<headers::Authorization<Bearer>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_jwt = authorization.token();

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::ES256);
    let token = match jsonwebtoken::decode(req_jwt, &JIA_JWT_SIGNING_KEY, &validation) {
        Ok(token) => token,
        Err(e) => {
            if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::Json(_)) {
                return Err((StatusCode::BAD_REQUEST, "invalid JWT payload".to_string()));
            } else {
                return Err((StatusCode::FORBIDDEN, "forbidden".to_string()));
            }
        }
    };

    let claims: Claims = token.claims;
    let jia_user_id = claims.jia_user_id;

    let user_repo = UserRepositoryImpl { pool: pool.0 };
    user_repo
        .insert(jia_user_id.to_string())
        .await
        .map_err(|e| {
            log::error!("user insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "user insert failed".to_string(),
            )
        })?;
    cookies.add(Cookie::new("jia_user_id", jia_user_id));

    Ok((StatusCode::OK, Json(vec!["Hello, world"])))
}

#[cfg(test)]
mod tests {
    use crate::model::cleaner::tests::Cleaner;
    use crate::test_helper;

    #[tokio::test]
    async fn test_get_index() -> Result<(), sqlx::Error> {
        let app = test_helper::spawn_app().await;
        let mut cleaner = Cleaner::new(app.database.clone());

        cleaner.prepare_table("user").await?;

        let client = reqwest::Client::new();
        let res = client
            .get(app.url.join("/").unwrap())
            .send()
            .await
            .expect("Failed to request");

        assert!(res.status().is_success());
        cleaner.clean().await?;

        Ok(())
    }
}
