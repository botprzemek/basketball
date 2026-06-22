use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::scoped_futures::ScopedFutureExt;
use uuid::Uuid;

use crate::adapter::repositories::{members, organizations};
use crate::domain::{entities::Member, ports::MemberPort};
use crate::{
    adapter::{
        providers::PostgresProvider,
        repositories::{accounts, organization::OrganizationRow},
    },
    domain::entities::Organization,
};

#[derive(Clone)]
pub struct MemberRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(account_id, organization_id))]
pub struct MemberRow {
    pub account_id: Uuid,
    pub organization_id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub gender: i32,
    pub birthdate: NaiveDate,
    pub picture: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl From<MemberRow> for Member {
    fn from(member: MemberRow) -> Self {
        Self {
            account_id: member.account_id,
            organization_id: member.organization_id,
            given_name: member.given_name,
            family_name: member.family_name,
            name: member.name,
            email: member.email,
            phone_number: member.phone_number,
            gender: member.gender,
            birthdate: member.birthdate,
            picture: member.picture,
            created_at: member.created_at,
            updated_at: member.updated_at,
            verified_at: member.verified_at,
        }
    }
}

impl MemberRepository {
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl MemberPort for MemberRepository {
    async fn select(&self, self_organization_id: Uuid) -> anyhow::Result<Vec<Member>> {
        let connection = &mut self.provider.get().await?;

        let rows = connection
            .build_transaction()
            .read_only()
            .run(|conn| {
                async move {
                    diesel::sql_query("RESET ALL").execute(conn).await?;

                    diesel::sql_query("SET auth.organization_id = $1")
                        .bind::<diesel::sql_types::Uuid, _>(self_organization_id)
                        .execute(conn)
                        .await?;

                    members::table
                        .select(MemberRow::as_select())
                        .get_results::<MemberRow>(conn)
                        .await
                }
                .scope_boxed()
            })
            .await?;

        let results = rows.into_iter().map(Member::from).collect::<Vec<_>>();

        Ok(results)
    }

    async fn select_by_account(
        &self,
        self_account_id: Uuid,
    ) -> anyhow::Result<Vec<(Member, Organization)>> {
        let connection = &mut self.provider.get().await?;

        let rows = connection
            .build_transaction()
            .read_only()
            .run(|conn| {
                async move {
                    diesel::sql_query("RESET ALL").execute(conn).await?;

                    diesel::sql_query("SET auth.account_id = $1")
                        .bind::<diesel::sql_types::Uuid, _>(self_account_id)
                        .execute(conn)
                        .await?;

                    members::table
                        .inner_join(organizations::table)
                        .select((MemberRow::as_select(), OrganizationRow::as_select()))
                        .get_results::<(MemberRow, OrganizationRow)>(conn)
                        .await
                }
                .scope_boxed()
            })
            .await?;

        let results = rows
            .into_iter()
            .map(|(member, organization)| (Member::from(member), Organization::from(organization)))
            .collect::<_>();

        Ok(results)
    }

    async fn select_by_self(
        &self,
        self_organization_id: Uuid,
        self_account_id: Uuid,
    ) -> anyhow::Result<Option<Member>> {
        let connection = &mut self.provider.get().await?;

        let row = connection
            .build_transaction()
            .read_only()
            .run(|conn| {
                async move {
                    diesel::sql_query("RESET ALL").execute(conn).await?;

                    diesel::sql_query("SET auth.account_id = $1")
                        .bind::<diesel::sql_types::Uuid, _>(self_account_id)
                        .execute(conn)
                        .await?;

                    diesel::sql_query("SET auth.organization_id = $1")
                        .bind::<diesel::sql_types::Uuid, _>(self_organization_id)
                        .execute(conn)
                        .await?;

                    members::table
                        .inner_join(accounts::table)
                        .select(MemberRow::as_select())
                        .first::<MemberRow>(conn)
                        .await
                        .optional()
                }
                .scope_boxed()
            })
            .await?;

        let result = row.map(Member::from);

        Ok(result)
    }
}
