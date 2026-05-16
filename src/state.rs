use std::sync::Arc;
use rok_auth::{Auth, AuthConfig};
use rok_auth::axum::{HasAuth, HasPool};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth: Auth,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        let auth = Auth::new(AuthConfig {
            secret: jwt_secret,
            ..Default::default()
        });
        Self { pool, auth }
    }
}

impl HasPool for AppState {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl HasAuth for AppState {
    fn auth_handle(&self) -> Arc<Auth> {
        Arc::new(self.auth.clone())
    }
}
