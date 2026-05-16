use rok_auth::Auth;
use rok_auth::axum::AuthLayer;

pub fn layer(auth: Auth) -> AuthLayer {
    AuthLayer::new(auth)
}
