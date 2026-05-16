use axum::Router;
use rok_testing::TestClient;

pub struct TestApp {
    pub client: TestClient,
}

impl TestApp {
    pub async fn boot() -> Self {
        let _ = dotenvy::from_filename(".env.test");
        TestApp {
            client: TestClient::new(Router::new()),
        }
    }
}
