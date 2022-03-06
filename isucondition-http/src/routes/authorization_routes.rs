use crate::requests::current_user_id::SESSION_USER_ID;
use crate::requests::session::{SessionID, SESSION_KEY};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use axum::extract::{Extension, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{headers, Json};
use isucondition_core::repos::repository_manager::RepositoryManager;
use isucondition_core::repos::user_repository::UserRepository;
use std::sync::Arc;
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

pub async fn post_authentication<Repo: RepositoryManager>(
    Extension(repo): Extension<Arc<Repo>>,
    TypedHeader(authorization): axum::extract::TypedHeader<headers::Authorization<Bearer>>,
    Extension(store): Extension<RedisSessionStore>,
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

    repo.user_repository()
        .insert(jia_user_id.to_string())
        .await
        .map_err(|e| {
            log::error!("user insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "user insert failed".to_string(),
            )
        })?;
    let mut session = Session::new();
    let result = session.insert(SESSION_USER_ID, jia_user_id.to_string());
    if result.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".to_string(),
        ));
    }
    let session_id = session.id().to_string();
    let result = store.store_session(session).await;
    if result.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".to_string(),
        ));
    }
    cookies.add(Cookie::new(SESSION_KEY, session_id));

    Ok((StatusCode::OK, Json(vec!["Hello, world"])))
}

pub async fn post_signout(
    cookies: Cookies,
    Extension(store): Extension<RedisSessionStore>,
    session_id: SessionID,
) -> impl IntoResponse {
    cookies.remove(Cookie::new("jia_user_id", ""));
    if session_id.is_none() {
        return (StatusCode::OK, Json("OK"));
    }
    let session_id = session_id.unwrap();
    let mut session = store.load_session(session_id).await;
    match session {
        Err(_) => {}
        Ok(session) => {
            if session.is_some() {
                let result = store.destroy_session(session.unwrap()).await;
                if result.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("internal server error"),
                    );
                }
            }
            cookies.remove(Cookie::new(SESSION_KEY, ""));
        }
    }

    (StatusCode::OK, Json("OK"))
}

#[cfg(test)]
mod tests {
    use crate::test_helper;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_post_signout() -> Result<(), anyhow::Error> {
        let app = test_helper::spawn_app().await;

        let client = reqwest::Client::new();
        let res = client
            .post(app.url.join("/api/signout").unwrap())
            .send()
            .await
            .expect("Failed to request");

        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }
}
