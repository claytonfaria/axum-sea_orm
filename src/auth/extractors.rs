use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
};
use headers::{authorization::Bearer, Authorization};

use super::{
    error::AuthError,
    jwt::{verify, Claims},
};

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::MissingCredentials)?;
        // Decode the user data
        let token_data = verify(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data)
    }
}
