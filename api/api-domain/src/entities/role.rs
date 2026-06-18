use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Role {
    pub fn new(organization_id: Uuid, name: String, description: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            organization_id,
            name,
            description,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
