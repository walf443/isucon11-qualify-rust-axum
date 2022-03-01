use crate::responses;
use crate::responses::error::Error::UnauthorizedError;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
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

#[async_trait]
impl<B> FromRequest<B> for CurrentUserID
where
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        Ok(Self::Some(UserID::new("1".to_string())))
    }
}
