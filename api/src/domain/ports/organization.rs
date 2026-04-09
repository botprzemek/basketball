use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Organization;

#[async_trait]
pub trait OrganizationPort: Clone + Send + Sync {
    async fn select_all(&self) -> anyhow::Result<Vec<Organization>>;
    async fn select_by_id(&self, id: Uuid) -> anyhow::Result<Option<Organization>>;
    async fn insert(&self, account: Organization) -> anyhow::Result<Organization>;
    async fn update(&self, account: Organization) -> anyhow::Result<Organization>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
