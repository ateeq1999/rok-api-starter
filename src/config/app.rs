use rok_config::Config;

#[derive(Config, Debug)]
pub struct AppConfig {
    #[env("APP_NAME", default = "rok-api")]
    pub name: String,
    #[env("LISTEN_ADDR", default = "0.0.0.0:3000")]
    pub listen_addr: String,
}
