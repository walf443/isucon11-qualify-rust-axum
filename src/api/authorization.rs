use crate::model::user_repository::{UserRepository, UserRepositoryImpl};
use crate::{IntoResponse, MySqlPool, StatusCode};
use axum::extract::TypedHeader;
use axum::headers::authorization::Bearer;
use axum::{extract, headers, Json};
use tower_cookies::{Cookie, Cookies};
use tracing::log;

const JIA_JWT_SIGNING_KEY_PATH: &str = "ec256-public.pem";

lazy_static::lazy_static! {
    static ref JIA_JWT_SIGNING_KEY_PEM: Vec<u8> = std::fs::read(JIA_JWT_SIGNING_KEY_PATH).expect("failed to read JIA JWT signing key file");
    static ref JIA_JWT_SIGNING_KEY: jsonwebtoken::DecodingKey = jsonwebtoken::DecodingKey::from_ec_pem(&JIA_JWT_SIGNING_KEY_PEM).expect("failed to parse JIA JWT signing key");
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

pub async fn post_signout(
    cookies: Cookies,
) -> impl IntoResponse {
    cookies.remove(Cookie::new("jia_user_id", ""));

    (StatusCode::OK, Json("OK"))
}

#[cfg(test)]
mod tests {
}