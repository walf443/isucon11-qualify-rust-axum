use async_redis_session::RedisSessionStore;
use async_session::SessionStore;
use async_trait::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts, TypedHeader};
use axum::headers::Cookie;

pub enum SessionID {
    Some(String),
    None,
}

impl SessionID {
    pub fn is_none(&self) -> bool {
        match self {
            SessionID::Some(_) => false,
            SessionID::None => true,
        }
    }
    pub fn unwrap(&self) -> String {
        match self {
            SessionID::Some(str) => str.clone(),
            SessionID::None => panic!("please use is_none before call unwrap"),
        }
    }
}

pub const SESSION_KEY: &str = "isucondition_rust";

#[async_trait]
impl<B> FromRequest<B> for SessionID
where
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie = cookie.as_ref().and_then(|cookie| cookie.get(SESSION_KEY));
        match session_cookie {
            None => Ok(Self::None),
            Some(session_id) => Ok(Self::Some(session_id.to_string())),
        }
    }
}
