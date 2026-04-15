use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::{Client, Row, Statement, types::Type};
use uuid::Uuid;

use crate::domain::{entities::Organization, ports::OrganizationPort};

#[derive(Clone)]
pub struct OrganizationRepository {
    client: Arc<Client>,
    select: Statement,
    select_by_self: Statement,
    insert: Statement,
    update: Statement,
    delete: Statement,
}

impl From<&Row> for Organization {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl OrganizationRepository {
    pub async fn new(client: Arc<Client>) -> anyhow::Result<Self> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS basketball.organizations (
                id UUID PRIMARY KEY,
                name STRING NOT NULL,
                slug STRING UNIQUE NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at TIMESTAMPTZ
            );
        
            CREATE INDEX IF NOT EXISTS idx_organizations_name ON basketball.organizations(name);
            CREATE INDEX IF NOT EXISTS idx_organizations_slug ON basketball.organizations(slug DESC);
            CREATE INDEX IF NOT EXISTS idx_organizations_created_at ON basketball.organizations(created_at DESC);
        ")
            .await?;

        let select = client
            .prepare_typed(
                "
                SELECT
                    id,
                    name,
                    slug,
                    is_active,
                    created_at,
                    updated_at
                FROM basketball.organizations
            ",
                &[],
            )
            .await?;

        let select_by_self = client
            .prepare_typed(
                "
                SELECT
                    id,
                    name,
                    slug,
                    is_active,
                    created_at,
                    updated_at
                FROM basketball.organizations
                WHERE id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        let insert = client
            .prepare_typed(
                "
                INSERT INTO basketball.organizations (
                    id,
                    name,
                    slug,
                    is_active,
                    created_at,
                    updated_at
                )
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                )
            ",
                &[
                    Type::UUID,
                    Type::TEXT,
                    Type::TEXT,
                    Type::BOOL,
                    Type::TIMESTAMPTZ,
                    Type::TIMESTAMPTZ,
                ],
            )
            .await?;

        let update = client
            .prepare_typed(
                "
                UPDATE basketball.organizations
                SET (
                    name,
                    slug,
                    is_active,
                    updated_at
                ) = (
                    $1,
                    $2,
                    $3,
                    $4
                )
            ",
                &[Type::TEXT, Type::TEXT, Type::BOOL, Type::TIMESTAMPTZ],
            )
            .await?;

        let delete = client
            .prepare_typed(
                "
                DELETE
                FROM basketball.organizations
                WHERE id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        Ok(Self {
            client,
            select,
            select_by_self,
            insert,
            update,
            delete,
        })
    }
}

#[async_trait]
impl OrganizationPort for OrganizationRepository {
    async fn select(&self) -> anyhow::Result<Vec<Organization>> {
        let result = self
            .client
            .query(&self.select.clone(), &[])
            .await?
            .iter()
            .map(Organization::from)
            .collect::<Vec<Organization>>();

        Ok(result)
    }

    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Organization>> {
        let result = self
            .client
            .query(&self.select_by_self.clone(), &[&id])
            .await?
            .first()
            .map(Organization::from);

        Ok(result)
    }

    async fn insert(&self, organization: Organization) -> anyhow::Result<Organization> {
        self.client
            .execute(
                &self.insert.clone(),
                &[
                    &organization.id,
                    &organization.name,
                    &organization.slug,
                    &organization.is_active,
                    &organization.created_at,
                    &organization.updated_at,
                ],
            )
            .await?;

        Ok(organization)
    }

    async fn update(&self, organization: Organization) -> anyhow::Result<Organization> {
        self.client
            .execute(
                &self.update.clone(),
                &[
                    &organization.name,
                    &organization.slug,
                    &organization.is_active,
                    &organization.updated_at,
                ],
            )
            .await?;

        Ok(organization)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.client.execute(&self.delete.clone(), &[&id]).await?;

        Ok(())
    }
}
