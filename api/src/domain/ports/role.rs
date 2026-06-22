use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Role;

#[async_trait]
pub trait RolePort: Clone + Send + Sync {
    async fn select(&self, organization_id: Uuid) -> anyhow::Result<Vec<Role>>;
    async fn select_by_member(
        &self,
        organization_id: Uuid,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<Role>>;
}
