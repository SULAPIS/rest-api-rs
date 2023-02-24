use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};

use crate::{error::AppError, models::auth::Claims, KEYS};
use jsonwebtoken::{decode, encode, Header, Validation};

pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time is wrong");

    let eight_from_now = since_the_epoch + Duration::from_secs(28800);

    eight_from_now.as_secs()
}

pub fn generate_token(email: &String) -> String {
    encode(&Header::default(), &email, &KEYS.encoding).unwrap()
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
