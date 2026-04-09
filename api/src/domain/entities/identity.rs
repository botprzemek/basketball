use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Identity {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Identity {
    pub fn new(organization_id: Uuid, account_id: Uuid, email: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            organization_id,
            account_id,
            email,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
