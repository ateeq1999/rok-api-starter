use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rok_auth::axum::Ctx;
use rok_auth::{password, Claims, MagicLink, MagicLinkConfig, PasswordReset};
use rok_encrypt::Encrypter;
use rok_orm::{Model, PgModel};
use rok_validate::Valid;
use serde_json::json;
use std::collections::HashMap;

use crate::app::models::User;
use crate::app::validators::auth_requests::*;
use crate::state::AppState;

pub struct AuthController;

impl AuthController {
    pub async fn register(
        State(state): State<AppState>,
        Valid(body): Valid<RegisterRequest>,
    ) -> impl IntoResponse {
        let exists = User::filter("email", body.email.as_str())
            .first()
            .await;
        match exists {
            Ok(Some(_)) => {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "Email already registered" })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
            Ok(None) => {}
        }

        let hash = match password::hash(&body.password) {
            Ok(h) => h,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        let user = match User::create_returning(
            &state.pool,
            &[("email", body.email.as_str().into()), ("password_hash", hash.into())],
        )
        .await
        {
            Ok(u) => u,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        let claims = Claims::new(user.id.to_string(), vec!["user"]);
        let token = match state.auth.sign(&claims) {
            Ok(t) => t,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        (
            StatusCode::CREATED,
            Json(json!({
                "token": token,
                "user": { "id": user.id, "email": user.email }
            })),
        )
    }

    pub async fn login(
        State(state): State<AppState>,
        Valid(body): Valid<LoginRequest>,
    ) -> impl IntoResponse {
        let user = match User::filter("email", body.email.as_str())
            .first()
            .await
        {
            Ok(Some(u)) => u,
            Ok(None) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Invalid credentials" })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        match password::verify(&body.password, &user.password_hash) {
            Ok(true) => {}
            Ok(false) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Invalid credentials" })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        }

        let claims = Claims::new(user.id.to_string(), vec!["user"]);
        let token = match state.auth.sign(&claims) {
            Ok(t) => t,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        (
            StatusCode::OK,
            Json(json!({
                "token": token,
                "user": { "id": user.id, "email": user.email }
            })),
        )
    }

    pub async fn me(ctx: Ctx) -> impl IntoResponse {
        let claims = match ctx.require_auth() {
            Ok(c) => c,
            Err(_) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Unauthorized" })),
                );
            }
        };

        let id: i64 = match claims.sub.parse() {
            Ok(id) => id,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid token subject" })),
                );
            }
        };

        let user = match User::find_by_pk(ctx.db(), id).await {
            Ok(Some(u)) => u,
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "User not found" })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        (
            StatusCode::OK,
            Json(json!({
                "user": {
                    "id": user.id,
                    "email": user.email,
                    "created_at": user.created_at,
                }
            })),
        )
    }

    pub async fn logout(_ctx: Ctx) -> impl IntoResponse {
        (StatusCode::OK, Json(json!({ "message": "Logged out" })))
    }

    pub async fn forgot_password(
        State(state): State<AppState>,
        Valid(body): Valid<ForgotPasswordRequest>,
    ) -> impl IntoResponse {
        let user = User::filter("email", body.email.as_str())
            .first()
            .await;

        if let Ok(Some(_)) = user {
            let _ = PasswordReset::issue(&state.pool, &body.email).await;
        }

        (
            StatusCode::OK,
            Json(json!({
                "message": "If the email exists, a reset link has been sent"
            })),
        )
    }

    pub async fn reset_password(
        State(state): State<AppState>,
        Valid(body): Valid<ResetPasswordRequest>,
    ) -> impl IntoResponse {
        let email = match PasswordReset::verify(&state.pool, &body.token).await {
            Ok(Some(email)) => email,
            Ok(None) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid or expired reset token" })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        let hash = match password::hash(&body.password) {
            Ok(h) => h,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        let result = User::update_where(
            &state.pool,
            User::query().where_eq("email", email.as_str()),
            &[("password_hash", hash.into())],
        )
        .await;

        match result {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({ "message": "Password updated" })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

    pub async fn magic_link(
        State(state): State<AppState>,
        Valid(body): Valid<MagicLinkRequest>,
    ) -> impl IntoResponse {
        let encrypter = Encrypter::from_config(
            rok_encrypt::EncryptConfig::new(&state.auth.config().secret),
        );
        let config = MagicLinkConfig::default();

        let token = MagicLink::issue(&encrypter, &body.email, &config);

        (StatusCode::OK, Json(json!({ "token": token })))
    }

    pub async fn magic_link_callback(
        State(state): State<AppState>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let token = match params.get("token") {
            Some(t) => t,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Missing token query param" })),
                );
            }
        };

        let encrypter = Encrypter::from_config(
            rok_encrypt::EncryptConfig::new(&state.auth.config().secret),
        );

        let email = match MagicLink::verify(&state.pool, &encrypter, token).await {
            Ok(e) => e,
            Err(e) => {
                let msg = match &e {
                    rok_auth::AuthError::TokenExpired => "Magic link expired",
                    rok_auth::AuthError::InvalidToken => "Invalid magic link",
                    _ => "Verification failed",
                };
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": msg })),
                );
            }
        };

        let user = match User::filter("email", email.as_str()).first().await {
            Ok(Some(u)) => u,
            Ok(None) => {
                let hash = match password::hash(&uuid::Uuid::new_v4().to_string()) {
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
                        ("email", email.as_str().into()),
                        ("password_hash", hash.into()),
                    ],
                )
                .await
                {
                    Ok(u) => u,
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "error": e.to_string() })),
                        );
                    }
                }
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        let claims = Claims::new(user.id.to_string(), vec!["user"]);
        let access_token = match state.auth.sign(&claims) {
            Ok(t) => t,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                );
            }
        };

        (
            StatusCode::OK,
            Json(json!({
                "token": access_token,
                "user": { "id": user.id, "email": user.email }
            })),
        )
    }
}
