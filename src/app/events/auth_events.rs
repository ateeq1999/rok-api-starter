use chrono::{DateTime, Utc};

pub struct UserRegistered {
    pub user_id: i64,
    pub email: String,
    pub at: DateTime<Utc>,
}

pub struct UserLoggedIn {
    pub user_id: i64,
    pub at: DateTime<Utc>,
}
