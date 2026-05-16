use rok_validate::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(required, email)]
    pub email: String,
    #[validate(required, min = 8, max = 128)]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
}
