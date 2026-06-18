use uuid::Uuid;

use crate::entities::Role;
use crate::ports::RolePort;

pub struct RoleApplication<O: RolePort> {
    role_service: O,
}

impl<O: RolePort> RoleApplication<O> {
    pub fn new(role_service: O) -> Self {
        Self { role_service }
    }

    pub async fn find(&self, organization_id: Uuid) -> anyhow::Result<Vec<Role>> {
        self.role_service.select(organization_id).await
    }

    pub async fn find_by_identity(
        &self,
        organization_id: Uuid,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<Role>> {
        self.role_service
            .select_by_identity(organization_id, account_id)
            .await
    }
}
