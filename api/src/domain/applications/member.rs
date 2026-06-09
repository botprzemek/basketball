use chrono::Utc;
use uuid::Uuid;

use crate::domain::{entities::{Member}, ports::{MemberPort}};

pub struct MemberApplication<M: MemberPort> {
    member_service: M,
}

impl<M: MemberPort> MemberApplication<M> {
    pub fn new(member_service: M) -> Self {
        Self { member_service }
    }

    pub fn find_all(&self, organization_id: Uuid) {
        self.member_service
    }

    pub fn find_by_identity(
        &self,
        organization_id: Uuid,
        account_id: Uuid
    ) {
        self.member_service
    }
}
