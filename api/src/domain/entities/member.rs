use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub account_id: Uuid,
    pub organization_id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub gender: i32,
    pub birthdate: NaiveDate,
    pub picture: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}
