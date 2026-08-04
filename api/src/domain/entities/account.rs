use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            email,
            password_hash,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            verified_at: None,
        }
    }
}
