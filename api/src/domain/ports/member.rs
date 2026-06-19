use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Member, Organization};

#[async_trait]
pub trait MemberPort: Clone + Send + Sync {
    async fn select(&self, organization_id: Uuid) -> anyhow::Result<Vec<Member>>;
    async fn select_by_account(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<(Member, Organization)>>;
    async fn select_by_self(
        &self,
        organization_id: Uuid,
        account_id: Uuid,
    ) -> anyhow::Result<Option<Member>>;
}
