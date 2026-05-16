use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rok_auth::axum::Ctx;
use rok_orm::PgModel;
use rok_validate::Valid;
use serde_json::json;

use crate::app::models::User;
use crate::app::validators::admin_requests::*;
use crate::state::AppState;

pub struct UserController;

impl UserController {
    pub async fn index(ctx: Ctx, State(state): State<AppState>) -> impl IntoResponse {
        if let Err(_) = ctx.require_auth() {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" })),
            );
        }

        match User::all(&state.pool).await {
            Ok(users) => {
                let data: Vec<_> = users
                    .into_iter()
                    .map(|u| {
                        json!({
                            "id": u.id,
                            "email": u.email,
                            "created_at": u.created_at,
                        })
                    })
                    .collect();
                (StatusCode::OK, Json(json!({ "data": data })))
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    pub async fn show(
        ctx: Ctx,
        State(state): State<AppState>,
        Path(id): Path<i64>,
    ) -> impl IntoResponse {
        if let Err(_) = ctx.require_auth() {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" })),
            );
        }

        match User::find_by_pk(&state.pool, id).await {
            Ok(Some(user)) => (
                StatusCode::OK,
                Json(json!({
                    "user": {
                        "id": user.id,
                        "email": user.email,
                        "created_at": user.created_at,
                    }
                })),
            ),
            Ok(None) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "User not found" })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    pub async fn store(
        State(state): State<AppState>,
        Valid(body): Valid<CreateUserRequest>,
    ) -> impl IntoResponse {
        let hash = match rok_auth::password::hash(&body.password) {
            Ok(h) => h,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        match User::create_returning(
            &state.pool,
            &[
                ("email", body.email.as_str().into()),
                ("password_hash", hash.into()),
            ],
        )
        .await
        {
            Ok(user) => (
                StatusCode::CREATED,
                Json(json!({
                    "user": { "id": user.id, "email": user.email }
                })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<i64>,
        Valid(body): Valid<UpdateUserRequest>,
    ) -> impl IntoResponse {
        let mut data: Vec<(&str, rok_orm::SqlValue)> = Vec::new();
        if let Some(email) = &body.email {
            data.push(("email", email.as_str().into()));
        }

        match User::update_by_pk(&state.pool, id, &data).await {
            Ok(count) if count > 0 => (
                StatusCode::OK,
                Json(json!({ "message": "User updated" })),
            ),
            Ok(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "User not found" })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    pub async fn destroy(
        State(state): State<AppState>,
        Path(id): Path<i64>,
    ) -> impl IntoResponse {
        match User::delete_by_pk(&state.pool, id).await {
            Ok(count) if count > 0 => (
                StatusCode::OK,
                Json(json!({ "message": "User deleted" })),
            ),
            Ok(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "User not found" })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }
}
