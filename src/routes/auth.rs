use axum::{
    routing::{get, post},
    Router,
};
use crate::app::controllers::auth_controller::AuthController;
use crate::state::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(AuthController::register))
        .route("/auth/login", post(AuthController::login))
        .route("/auth/logout", post(AuthController::logout))
        .route("/auth/me", get(AuthController::me))
        .route("/auth/forgot-password", post(AuthController::forgot_password))
        .route("/auth/reset-password", post(AuthController::reset_password))
        .route("/auth/magic-link", post(AuthController::magic_link))
        .route(
            "/auth/magic-link/callback",
            get(AuthController::magic_link_callback),
        )
}
