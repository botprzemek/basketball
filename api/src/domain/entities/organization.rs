use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Organization {
    pub fn new(name: String, slug: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            slug,
            is_active: true,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
