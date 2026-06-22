use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::adapter::{providers::PostgresProvider, repositories::organizations::dsl::*};
use crate::domain::{entities::Organization, ports::OrganizationPort};

#[derive(Clone)]
pub struct OrganizationRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrganizationRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<OrganizationRow> for Organization {
    fn from(organization: OrganizationRow) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
            deleted_at: organization.deleted_at,
        }
    }
}

impl From<Organization> for OrganizationRow {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
            deleted_at: organization.deleted_at,
        }
    }
}

impl OrganizationRepository {
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl OrganizationPort for OrganizationRepository {
    async fn select_by_self(&self, self_id: Uuid) -> anyhow::Result<Option<Organization>> {
        let connection = &mut self.provider.get().await?;
        let result = organizations
            .select(OrganizationRow::as_select())
            .filter(id.eq(self_id))
            .first(connection)
            .await
            .optional()?
            .map(Organization::from);

        Ok(result)
    }
}
