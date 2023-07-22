use axum::{
    http::{request::Parts, header::AUTHORIZATION},

    extract::{FromRequestParts, FromRef}
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::db::{User, get_user};
use crate::state::AppState;
use crate::error::{Result, AppError};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
    name: String,
}

#[derive(Debug)]
pub struct Auth(pub Option<User>);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Auth 
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, st: &S) -> Result<Self> {
        let state = AppState::from_ref(st);  
        match parts.headers.get(AUTHORIZATION) {
            Some(token) => {
                tracing::info!("token: {:?}", token);
                let jwt = jsonwebtoken::decode::<Claims>(
                    token.to_str()?, 
                    &DecodingKey::from_secret(state.signing_key.as_bytes()), 
                    &Validation::new(jsonwebtoken::Algorithm::HS256))?;
                let user = get_user(jwt.claims.name, state.pool).await?;
                return Ok(Self(Some(user)))
            }
            None => { return Ok(Self(None))}
        }
    }
}
