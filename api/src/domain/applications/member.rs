use uuid::Uuid;

use crate::domain::{entities::Member, ports::MemberPort};

pub struct MemberApplication<M: MemberPort> {
    member_service: M,
}

impl<M: MemberPort> MemberApplication<M> {
    pub fn new(member_service: M) -> Self {
        Self { member_service }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Member>> {
        self.member_service.select().await
    }

    pub async fn find_by_organization(&self, organization_id: Uuid) -> anyhow::Result<Vec<Member>> {
        self.member_service
            .select_by_organization(organization_id)
            .await
    }
}
