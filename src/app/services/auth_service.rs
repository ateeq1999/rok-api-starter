use rok_auth::{password, Auth, AuthError, Claims, TokenPair};
use rok_orm::PgModel;
use sqlx::PgPool;

use crate::app::models::User;

pub struct AuthService;

impl AuthService {
    pub async fn attempt(
        auth: &Auth,
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<TokenPair, AuthError> {
        let user = User::filter("email", email)
            .first()
            .await
            .map_err(|e: sqlx::Error| AuthError::Internal(e.to_string()))?
            .ok_or(AuthError::InvalidCredentials)?;

        let valid = password::verify(password, &user.password_hash)
            .map_err(|e| AuthError::HashError(e.to_string()))?;

        if !valid {
            return Err(AuthError::InvalidCredentials);
        }

        let access_token = auth.sign(&Claims::new(user.id.to_string(), vec!["user"]))?;
        let refresh_token = auth.sign_refresh(&user.id.to_string())?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
