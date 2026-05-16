use axum::Router;
use rok_orm::OrmLayer;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod app;
mod config;
mod routes;
mod state;

use config::{AppConfig, AuthConfig, DatabaseConfig};
use routes::{api_router, auth_router};
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_cfg = AppConfig::load();
    let auth_cfg = AuthConfig::load();
    let db_cfg = DatabaseConfig::load();

    let pool = PgPoolOptions::new()
        .max_connections(db_cfg.max_connections)
        .connect(&db_cfg.url)
        .await
        .expect("failed to connect to database");

    let state = AppState::new(pool.clone(), auth_cfg.jwt_secret);

    let app = Router::new()
        .merge(auth_router())
        .merge(api_router())
        .layer(OrmLayer::new(pool.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&app_cfg.listen_addr)
        .await
        .expect("failed to bind");

    println!("{} listening on {}", app_cfg.name, app_cfg.listen_addr);
    axum::serve(listener, app).await.expect("server error");
}
