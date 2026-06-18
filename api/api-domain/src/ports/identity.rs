use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::{Identity, Organization};

#[async_trait]
pub trait IdentityPort: Clone + Send + Sync {
    async fn select(&self) -> anyhow::Result<Vec<Identity>>;
    async fn select_by_account(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<(Identity, Organization)>>;
    async fn insert(&self, identity: Identity) -> anyhow::Result<Identity>;
    async fn delete(&self, account_id: Uuid, organization_id: Uuid) -> anyhow::Result<()>;
}
