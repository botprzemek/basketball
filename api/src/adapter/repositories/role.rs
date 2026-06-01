use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::domain::ports::RolePort;
use crate::{
    adapter::{
        providers::PostgresProvider,
        repositories::{identities_roles, roles},
    },
    domain::entities::Role,
};

#[derive(Clone)]
pub struct RoleRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RoleRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<RoleRow> for Role {
    fn from(role: RoleRow) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<Role> for RoleRow {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl RoleRepository {
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl RolePort for RoleRepository {
    async fn select_by_organization(
        &self,
        self_organization_id: Uuid,
    ) -> anyhow::Result<Vec<Role>> {
        let connection = &mut self.provider.get().await?;
        let result = identities_roles::table
            .distinct()
            .inner_join(roles::table)
            .filter(identities_roles::organization_id.eq(self_organization_id))
            .select(RoleRow::as_select())
            .get_results::<RoleRow>(connection)
            .await?
            .into_iter()
            .map(Role::from)
            .collect::<Vec<_>>();

        Ok(result)
    }

    async fn select_by_identity(
        &self,
        self_account_id: Uuid,
        self_organization_id: Uuid,
    ) -> anyhow::Result<Vec<Role>> {
        let connection = &mut self.provider.get().await?;
        let result = identities_roles::table
            .inner_join(roles::table)
            .filter(identities_roles::organization_id.eq(self_organization_id))
            .filter(identities_roles::account_id.eq(self_account_id))
            .select(RoleRow::as_select())
            .get_results::<RoleRow>(connection)
            .await?
            .into_iter()
            .map(Role::from)
            .collect::<Vec<_>>();

        Ok(result)
    }
}
