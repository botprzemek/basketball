use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Identity;

#[async_trait]
pub trait IdentityPort: Clone + Send + Sync {
    async fn select_all(&self) -> anyhow::Result<Vec<Identity>>;
    async fn select_by_id(&self, id: Uuid) -> anyhow::Result<Option<Identity>>;
    async fn insert(&self, account: Identity) -> anyhow::Result<Identity>;
    async fn update(&self, account: Identity) -> anyhow::Result<Identity>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
