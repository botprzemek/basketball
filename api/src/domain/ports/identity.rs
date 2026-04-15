use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{AccountIdentity, Identity};

#[async_trait]
pub trait IdentityPort: Clone + Send + Sync {
    async fn select(&self) -> anyhow::Result<Vec<Identity>>;
    async fn select_by_self(&self, identity_id: Uuid) -> anyhow::Result<Option<Identity>>;
    async fn select_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<AccountIdentity>>;
    async fn select_by_account_identity(
        &self,
        account_id: Uuid,
        identity_id: Uuid,
    ) -> anyhow::Result<Option<AccountIdentity>>;
    async fn insert(&self, identity: Identity) -> anyhow::Result<Identity>;
    async fn update(&self, identity: Identity) -> anyhow::Result<Identity>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
