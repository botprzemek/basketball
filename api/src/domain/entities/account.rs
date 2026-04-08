use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(password_hash: String, first_name: String, last_name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            password_hash,
            first_name,
            last_name,
            is_active: true,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
