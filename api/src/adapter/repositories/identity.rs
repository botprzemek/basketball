use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::{Client, Row, Statement, types::Type};
use uuid::Uuid;

use crate::domain::{
    entities::{AccountIdentity, Identity},
    ports::IdentityPort,
};

#[derive(Clone)]
pub struct IdentityRepository {
    client: Arc<Client>,
    select: Statement,
    select_by_self: Statement,
    select_by_account: Statement,
    select_by_account_identity: Statement,
    insert: Statement,
    update: Statement,
    delete: Statement,
}

impl From<&Row> for Identity {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            account_id: row.get("account_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&Row> for AccountIdentity {
    fn from(row: &Row) -> Self {
        Self {
            identity_id: row.get("identity_id"),
            organization_id: row.get("organization_id"),
            organization_name: row.get("organization_name"),
            organization_slug: row.get("organization_slug"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl IdentityRepository {
    pub async fn new(client: Arc<Client>) -> anyhow::Result<Self> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS basketball.identities (
                id UUID PRIMARY KEY,
                organization_id UUID NOT NULL REFERENCES basketball.organizations(id) ON DELETE CASCADE,
                account_id UUID NOT NULL REFERENCES basketball.accounts(id) ON DELETE CASCADE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at TIMESTAMPTZ,
                UNIQUE(organization_id, account_id)
            );

            CREATE INDEX IF NOT EXISTS idx_identities_organization_id ON basketball.identities(organization_id);
            CREATE INDEX IF NOT EXISTS idx_identities_account_id ON basketball.identities(account_id);
            CREATE INDEX IF NOT EXISTS idx_identities_created_at ON basketball.identities(created_at DESC);
        ")
            .await?;

        let select = client
            .prepare_typed(
                "
                SELECT
                    id,
                    organization_id,
                    account_id,
                    created_at,
                    updated_at
                FROM basketball.identities
            ",
                &[],
            )
            .await?;

        let select_by_self = client
            .prepare_typed(
                "
                SELECT
                    id,
                    organization_id,
                    account_id,
                    created_at,
                    updated_at
                FROM basketball.identities
                WHERE basketball.identities.id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        let select_by_account = client
            .prepare_typed(
                "
                SELECT
                    basketball.identities.id AS identity_id,
                    basketball.identities.organization_id AS organization_id,
                    basketball.organizations.name AS organization_name,
                    basketball.organizations.slug AS organization_slug,
                    basketball.identities.created_at AS created_at,
                    basketball.identities.updated_at AS updated_at
                FROM
                    basketball.identities,
                    basketball.organizations
                WHERE basketball.identities.organization_id = basketball.organizations.id
                AND basketball.identities.account_id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        let select_by_account_identity = client
            .prepare_typed(
                "
                SELECT
                    basketball.identities.id AS identity_id,
                    basketball.identities.organization_id AS organization_id,
                    basketball.organizations.name AS organization_name,
                    basketball.organizations.slug AS organization_slug,
                    basketball.identities.created_at AS created_at,
                    basketball.identities.updated_at AS updated_at
                FROM
                    basketball.identities,
                    basketball.organizations
                WHERE basketball.identities.organization_id = basketball.organizations.id
                AND basketball.identities.account_id = $1
                AND basketball.identities.id = $2
            ",
                &[Type::UUID, Type::UUID],
            )
            .await?;

        let insert = client
            .prepare_typed(
                "
                INSERT INTO basketball.identities (
                    id,
                    organization_id,
                    account_id,
                    created_at,
                    updated_at
                )
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5
                )
            ",
                &[
                    Type::UUID,
                    Type::UUID,
                    Type::UUID,
                    Type::TIMESTAMPTZ,
                    Type::TIMESTAMPTZ,
                ],
            )
            .await?;

        let update = client
            .prepare_typed(
                "
                UPDATE basketball.identities
                SET (
                    organization_id,
                    account_id,
                    updated_at
                ) = (
                    $1,
                    $2,
                    $3
                )
            ",
                &[Type::UUID, Type::UUID, Type::TIMESTAMPTZ],
            )
            .await?;

        let delete = client
            .prepare_typed(
                "
                DELETE
                FROM basketball.identities
                WHERE basketball.identities.id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        Ok(Self {
            client,
            select,
            select_by_self,
            select_by_account,
            select_by_account_identity,
            insert,
            update,
            delete,
        })
    }
}

#[async_trait]
impl IdentityPort for IdentityRepository {
    async fn select(&self) -> anyhow::Result<Vec<Identity>> {
        let result = self
            .client
            .query(&self.select.clone(), &[])
            .await?
            .iter()
            .map(Identity::from)
            .collect::<Vec<Identity>>();

        Ok(result)
    }

    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Identity>> {
        let result = self
            .client
            .query(&self.select_by_self.clone(), &[&id])
            .await?
            .first()
            .map(Identity::from);

        Ok(result)
    }

    async fn select_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<AccountIdentity>> {
        let result = self
            .client
            .query(&self.select_by_account.clone(), &[&account_id])
            .await?
            .iter()
            .map(AccountIdentity::from)
            .collect::<Vec<AccountIdentity>>();

        Ok(result)
    }

    async fn select_by_account_identity(
        &self,
        account_id: Uuid,
        identity_id: Uuid,
    ) -> anyhow::Result<Option<AccountIdentity>> {
        let result = self
            .client
            .query(
                &self.select_by_account_identity.clone(),
                &[&account_id, &identity_id],
            )
            .await?
            .first()
            .map(AccountIdentity::from);

        Ok(result)
    }

    async fn insert(&self, identity: Identity) -> anyhow::Result<Identity> {
        self.client
            .execute(
                &self.insert.clone(),
                &[
                    &identity.id,
                    &identity.organization_id,
                    &identity.account_id,
                    &identity.created_at,
                    &identity.updated_at,
                ],
            )
            .await?;

        Ok(identity)
    }

    async fn update(&self, identity: Identity) -> anyhow::Result<Identity> {
        self.client
            .execute(
                &self.update.clone(),
                &[
                    &identity.organization_id,
                    &identity.account_id,
                    &identity.updated_at,
                ],
            )
            .await?;

        Ok(identity)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.client.execute(&self.delete.clone(), &[&id]).await?;

        Ok(())
    }
}
