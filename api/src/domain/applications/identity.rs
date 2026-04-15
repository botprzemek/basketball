use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{
    entities::{AccountIdentity, Identity},
    ports::IdentityPort,
};

pub struct IdentityApplication<O: IdentityPort> {
    identity_service: O,
}

#[derive(Deserialize)]
pub struct CreateIdentity {
    pub organization_id: Uuid,
    pub account_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateIdentity {
    pub organization_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
}

impl<O: IdentityPort> IdentityApplication<O> {
    pub fn new(identity_service: O) -> Self {
        Self { identity_service }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Identity>> {
        self.identity_service.select().await
    }

    pub async fn find_by_identity(&self, identity_id: Uuid) -> anyhow::Result<Option<Identity>> {
        self.identity_service.select_by_self(identity_id).await
    }

    pub async fn find_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<AccountIdentity>> {
        self.identity_service.select_by_account(account_id).await
    }

    pub async fn find_by_account_identity(
        &self,
        account_id: Uuid,
        identity_id: Uuid,
    ) -> anyhow::Result<Option<AccountIdentity>> {
        self.identity_service
            .select_by_account_identity(account_id, identity_id)
            .await
    }

    pub async fn create(&self, command: CreateIdentity) -> anyhow::Result<Identity> {
        let identity = Identity::new(command.organization_id, command.account_id);

        self.identity_service.insert(identity).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        command: UpdateIdentity,
    ) -> anyhow::Result<Option<Identity>> {
        let mut identity = match self.identity_service.select_by_self(id).await? {
            Some(identity) => identity,
            None => return Ok(None),
        };

        let mut has_changed = false;

        if let Some(organization_id) = command.organization_id {
            identity.organization_id = organization_id;
            has_changed = true;
        }

        if let Some(account_id) = command.account_id {
            identity.account_id = account_id;
            has_changed = true;
        }

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
