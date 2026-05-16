use rok_validate::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(required, email)]
    pub email: String,
    #[validate(required, min = 8, max = 128)]
    pub password: String,
    #[validate(required, same = "password")]
    pub password_confirmation: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(required, email)]
    pub email: String,
    #[validate(required, min = 8)]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(required, email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(required)]
    pub token: String,
    #[validate(required, min = 8, max = 128)]
    pub password: String,
    #[validate(required, same = "password")]
    pub password_confirmation: String,
}

#[derive(Deserialize, Validate)]
pub struct MagicLinkRequest {
    #[validate(required, email)]
    pub email: String,
}
