use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::scoped_futures::ScopedFutureExt;
use uuid::Uuid;

use crate::adapter::{
    providers::PostgresProvider,
    repositories::{account::AccountRow, accounts, identities, identity::IdentityRow},
};
use crate::domain::{entities::Member, ports::MemberPort};

#[derive(Clone)]
pub struct MemberRepository {
    provider: Arc<PostgresProvider>,
}

impl From<(AccountRow, IdentityRow)> for Member {
    fn from((account, identity): (AccountRow, IdentityRow)) -> Self {
        Self {
            id: account.id,
            organization_id: identity.organization_id,
            email: account.email,
            first_name: account.first_name,
            last_name: account.last_name,
            joined_at: identity.created_at,
            updated_at: identity.updated_at,
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

                    identities::table
                        .distinct()
                        .inner_join(accounts::table)
                        // .filter(identities::organization_id.eq(self_organization_id))
                        .select((AccountRow::as_select(), IdentityRow::as_select()))
                        .get_results::<(AccountRow, IdentityRow)>(conn)
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

                    identities::table
                        .distinct()
                        .inner_join(accounts::table)
                        .select((AccountRow::as_select(), IdentityRow::as_select()))
                        .first::<(AccountRow, IdentityRow)>(conn)
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
