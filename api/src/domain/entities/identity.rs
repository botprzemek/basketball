use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentity {
    pub identity_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Identity {
    pub fn new(organization_id: Uuid, account_id: Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            organization_id,
            account_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
