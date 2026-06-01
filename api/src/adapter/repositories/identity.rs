use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::domain::{entities::Identity, ports::IdentityPort};
use crate::{
    adapter::{
        providers::PostgresProvider,
        repositories::{
            identities::{self, dsl::*},
            organization::OrganizationRow,
            organizations,
        },
    },
    domain::entities::Organization,
};

#[derive(Clone)]
pub struct IdentityRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(account_id, organization_id))]
pub struct IdentityRow {
    pub account_id: Uuid,
    pub organization_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<IdentityRow> for Identity {
    fn from(identity: IdentityRow) -> Self {
        Self {
            organization_id: identity.organization_id,
            account_id: identity.account_id,
            created_at: identity.created_at,
            updated_at: identity.updated_at,
        }
    }
}

impl From<Identity> for IdentityRow {
    fn from(identity: Identity) -> Self {
        Self {
            organization_id: identity.organization_id,
            account_id: identity.account_id,
            created_at: identity.created_at,
            updated_at: identity.updated_at,
        }
    }
}

impl IdentityRepository {
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl IdentityPort for IdentityRepository {
    async fn select(&self) -> anyhow::Result<Vec<Identity>> {
        let connection = &mut self.provider.get().await?;
        let results = identities
            .select(IdentityRow::as_select())
            .get_results::<IdentityRow>(connection)
            .await?
            .into_iter()
            .map(Identity::from)
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn select_by_account(
        &self,
        self_account_id: Uuid,
    ) -> anyhow::Result<Vec<(Identity, Organization)>> {
        let connection = &mut self.provider.get().await?;

        let results = identities::table
            .inner_join(organizations::table)
            .filter(identities::account_id.eq(self_account_id))
            .select((IdentityRow::as_select(), OrganizationRow::as_select()))
            .get_results::<(IdentityRow, OrganizationRow)>(connection)
            .await?
            .into_iter()
            .map(|(identity, organization)| {
                (Identity::from(identity), Organization::from(organization))
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn select_by_self(
        &self,
        self_account_id: Uuid,
        self_organization_id: Uuid,
    ) -> anyhow::Result<Option<Identity>> {
        let connection = &mut self.provider.get().await?;
        let result = identities
            .select(IdentityRow::as_select())
            .filter(organization_id.eq(self_organization_id))
            .filter(account_id.eq(self_account_id))
            .first(connection)
            .await
            .optional()?
            .map(Identity::from);

        Ok(result)
    }

    async fn insert(&self, identity: Identity) -> anyhow::Result<Identity> {
        let connection = &mut self.provider.get().await?;
        let result = diesel::insert_into(identities)
            .values(IdentityRow::from(identity))
            .get_result::<IdentityRow>(connection)
            .await
            .map(Identity::from)?;

        Ok(result)
    }

    async fn delete(
        &self,
        self_account_id: Uuid,
        self_organization_id: Uuid,
    ) -> anyhow::Result<()> {
        let connection = &mut self.provider.get().await?;
        let _result = diesel::delete(
            identities
                .filter(organization_id.eq(self_organization_id))
                .filter(account_id.eq(self_account_id)),
        )
        .execute(connection)
        .await?;

        Ok(())
    }
}
