use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Organization;

#[async_trait]
pub trait OrganizationPort: Clone + Send + Sync {
    async fn select(&self) -> anyhow::Result<Vec<Organization>>;
    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Organization>>;
    async fn insert(&self, organization: Organization) -> anyhow::Result<Organization>;
    async fn update(&self, organziation: Organization) -> anyhow::Result<Organization>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
