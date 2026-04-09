use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entities::Identity,
    ports::IdentityPort,
};

pub struct IdentityApplication<O: IdentityPort> {
    identity_service: O,
}

impl<O: IdentityPort> IdentityApplication<O> {
    pub fn new(identity_service: O) -> Self {
        Self {
            identity_service,
        }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Identity>> {
        self.identity_service.select_all().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Identity>> {
        self.identity_service.select_by_id(id).await
    }

    pub async fn create(
        &self,
        organization_id: Uuid,
        account_id: Uuid,
        email: String,
    ) -> anyhow::Result<Identity> {
        let identity = Identity::new(organization_id, account_id, email);

        self.identity_service.insert(identity).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        slug: Option<String>,
    ) -> anyhow::Result<Option<Identity>> {
        let mut identity = match self.identity_service.select_by_id(id).await? {
            Some(identity) => identity,
            None => return Ok(None),
        };

        let mut has_changed = false;

        if has_changed {
            identity.updated_at = Some(Utc::now());
            identity = self.identity_service.update(identity).await?;
        }

        Ok(Some(identity))
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.identity_service.delete(id).await
    }
}
