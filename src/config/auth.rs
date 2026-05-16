use rok_config::Config;

#[derive(Config, Debug)]
pub struct AuthConfig {
    #[env("JWT_SECRET")]
    pub jwt_secret: String,
}
