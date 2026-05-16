use rok_orm::Model;
use serde::Serialize;

#[derive(Debug, Clone, Model, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
