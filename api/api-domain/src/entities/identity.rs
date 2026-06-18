use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub organization_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Identity {
    pub fn new(organization_id: Uuid, account_id: Uuid) -> Self {
        Self {
            organization_id,
            account_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
