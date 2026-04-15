use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Member;

#[async_trait]
pub trait MemberPort: Clone + Send + Sync {
    async fn select(&self) -> anyhow::Result<Vec<Member>>;
    async fn select_by_organization(&self, organization_id: Uuid) -> anyhow::Result<Vec<Member>>;
}
