use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::adapter::{providers::PostgresProvider, repositories::accounts::dsl::*};
use crate::domain::{entities::Member, ports::MemberPort};

#[derive(Clone)]
pub struct MemberRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<AccountRow> for Member {
    fn from(account: AccountRow) -> Self {
        Self {
            id: account.id,
            email: account.email,
            password_hash: account.password_hash,
            first_name: account.first_name,
            last_name: account.last_name,
            is_active: account.is_active,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

impl From<Account> for AccountRow {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            email: account.email,
            password_hash: account.password_hash,
            first_name: account.first_name,
            last_name: account.last_name,
            is_active: account.is_active,
            created_at: account.created_at,
            updated_at: account.updated_at,
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

        let rows = connection.build_transaction().read_only().run(| conn | {
            async move {
                diesel::sql_query("SELECT set_config('app.current_organization_id', $1, true)")
                    .bind::<diesel::sql_types::Uuid, _>(self_organization_id)
                    .execute(conn)
                    .await?;

                accounts::table
                    .distinct()
                    .filter(roles::organization_id.eq(self_organization_id))
                    .select(RoleRow::as_select())
                    .get_results::<RoleRow>(conn)
                    .await
            }.scope_boxed()
        }).await?;

        let results = rows.into_iter().map(Member::from).collect::<Vec<_>>();

        Ok(results)
    }

    async fn select_by_self(&self, self_organization_id: Uuid, self_id: Uuid) -> anyhow::Result<Option<Member>> {
        let connection = &mut self.provider.get().await?;
        let result = vec![];

        Ok(result)
    }
}
