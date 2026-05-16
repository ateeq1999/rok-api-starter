use axum::Router;
use rok_orm::OrmLayer;
use rok_testing::TestClient;
use sqlx::postgres::PgPoolOptions;

use rok_api_test::config::DatabaseConfig;
use rok_api_test::routes::{api_router, auth_router};
use rok_api_test::state::AppState;

pub struct TestApp {
    pub client: TestClient,
}

impl TestApp {
    pub async fn boot() -> Self {
        let _ = dotenvy::from_filename(".env.test");
        let db_cfg = DatabaseConfig::load();
        let pool = PgPoolOptions::new()
            .max_connections(db_cfg.max_connections)
            .connect(&db_cfg.url)
            .await
            .expect("failed to connect to database");

        let state = AppState::new(pool.clone(), "test-secret".to_string());

        let app = Router::new()
            .merge(auth_router())
            .merge(api_router())
            .layer(OrmLayer::new(pool))
            .with_state(state);

        TestApp {
            client: TestClient::new(app),
        }
    }
}
