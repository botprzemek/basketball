use uuid::Uuid;

use crate::domain::entities::Organization;
use crate::domain::ports::OrganizationPort;

pub struct OrganizationApplication<O: OrganizationPort> {
    organization_service: O,
}

impl<O: OrganizationPort> OrganizationApplication<O> {
    pub fn new(organization_service: O) -> Self {
        Self {
            organization_service,
        }
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Organization>> {
        self.organization_service.select_by_self(id).await
    }
}
