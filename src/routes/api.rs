use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::app::controllers::user_controller::UserController;
use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/users", get(UserController::index))
        .route("/api/v1/users", post(UserController::store))
        .route("/api/v1/users/:id", get(UserController::show))
        .route("/api/v1/users/:id", put(UserController::update))
        .route("/api/v1/users/:id", delete(UserController::destroy))
}
