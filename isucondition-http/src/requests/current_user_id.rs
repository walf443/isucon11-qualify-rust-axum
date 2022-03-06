use crate::requests::session::SessionID;
use crate::responses;
use crate::responses::error::Error::UnauthorizedError;
use async_redis_session::RedisSessionStore;
use async_session::SessionStore;
use async_trait::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts, TypedHeader};
use axum::headers::Cookie;
use isucondition_core::models::user::UserID;

pub enum CurrentUserID {
    Some(UserID),
    None,
}

impl CurrentUserID {
    pub fn is_none(&self) -> bool {
        match self {
            CurrentUserID::Some(_) => false,
            CurrentUserID::None => true,
        }
    }
    pub fn try_unwrap(&self) -> Result<UserID, responses::error::Error> {
        match self {
            CurrentUserID::Some(user_id) => Ok(user_id.clone()),
            CurrentUserID::None => Err(UnauthorizedError()),
        }
    }
}

pub const SESSION_USER_ID: &str = "jia_user_id";

#[async_trait]
impl<B> FromRequest<B> for CurrentUserID
where
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<RedisSessionStore>::from_request(req)
            .await
            .expect("session store not found");
        let session_id = SessionID::from_request(req).await?;
        if session_id.is_none() {
            return Ok(Self::None);
        }
        let session_id = session_id.unwrap();
        let session = store.load_session(session_id).await;

        return match session {
            Err(_) => {
                // TODO: it should be Err
                Ok(Self::None)
            }
            Ok(session) => {
                if session.is_none() {
                    return Ok(Self::None);
                }
                let current_user_id = session.unwrap().get::<String>(SESSION_USER_ID);
                match current_user_id {
                    None => Ok(CurrentUserID::None),
                    Some(user_id) => Ok(Self::Some(UserID::new(user_id.clone()))),
                }
            }
        };
    }
}
