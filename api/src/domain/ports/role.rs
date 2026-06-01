use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Role;

#[async_trait]
pub trait RolePort: Clone + Send + Sync {
    async fn select_by_organization(&self, organization_id: Uuid) -> anyhow::Result<Vec<Role>>;
    async fn select_by_identity(
        &self,
        account_id: Uuid,
        organization_id: Uuid,
    ) -> anyhow::Result<Vec<Role>>;
}
