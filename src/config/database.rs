use rok_config::Config;

#[derive(Config, Debug)]
pub struct DatabaseConfig {
    #[env("DATABASE_URL")]
    pub url: String,
    #[env("DB_MAX_CONNECTIONS", default = 10)]
    pub max_connections: u32,
}
