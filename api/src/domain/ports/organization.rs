use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Organization;

#[async_trait]
pub trait OrganizationPort: Clone + Send + Sync {
    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Organization>>;
}
