use uuid::Uuid;

use crate::domain::entities::{Member, Organization};
use crate::domain::ports::MemberPort;

pub struct MemberApplication<M: MemberPort> {
    member_service: M,
}

impl<M: MemberPort> MemberApplication<M> {
    pub fn new(member_service: M) -> Self {
        Self { member_service }
    }

    pub async fn find_all(&self, organization_id: Uuid) -> anyhow::Result<Vec<Member>> {
        self.member_service.select(organization_id).await
    }

    pub async fn find_by_account(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<(Member, Organization)>> {
        self.member_service.select_by_account(account_id).await
    }

    pub async fn find_by_self(
        &self,
        organization_id: Uuid,
        account_id: Uuid,
    ) -> anyhow::Result<Option<Member>> {
        self.member_service
            .select_by_self(organization_id, account_id)
            .await
    }
}
