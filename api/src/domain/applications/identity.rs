use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{
    entities::{Identity, Organization},
    ports::IdentityPort,
};

pub struct IdentityApplication<O: IdentityPort> {
    identity_service: O,
}

#[derive(Deserialize)]
pub struct CreateIdentity {
    pub account_id: Uuid,
    pub organization_id: Uuid,
}

#[derive(Deserialize)]
pub struct DeleteIdentity {
    pub account_id: Uuid,
    pub organization_id: Uuid,
}

impl<O: IdentityPort> IdentityApplication<O> {
    pub fn new(identity_service: O) -> Self {
        Self { identity_service }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Identity>> {
        self.identity_service.select().await
    }

    pub async fn find_by_account(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<Vec<(Identity, Organization)>> {
        self.identity_service.select_by_account(account_id).await
    }

    pub async fn create(&self, command: CreateIdentity) -> anyhow::Result<Identity> {
        let identity = Identity::new(command.account_id, command.organization_id);

        self.identity_service.insert(identity).await
    }

    pub async fn delete(&self, command: DeleteIdentity) -> anyhow::Result<()> {
        self.identity_service
            .delete(command.account_id, command.organization_id)
            .await
    }
}
