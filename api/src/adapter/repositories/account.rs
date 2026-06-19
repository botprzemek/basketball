use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::adapter::{providers::PostgresProvider, repositories::accounts::dsl::*};
use crate::domain::{entities::Account, ports::AccountPort};

#[derive(Clone)]
pub struct AccountRepository {
    provider: Arc<PostgresProvider>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::adapter::repositories::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl From<AccountRow> for Account {
    fn from(account: AccountRow) -> Self {
        Self {
            id: account.id,
            email: account.email,
            password_hash: account.password_hash,
            created_at: account.created_at,
            updated_at: account.updated_at,
            deleted_at: account.deleted_at,
            verified_at: account.verified_at,
        }
    }
}

impl From<Account> for AccountRow {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            email: account.email,
            password_hash: account.password_hash,
            created_at: account.created_at,
            updated_at: account.updated_at,
            deleted_at: account.deleted_at,
            verified_at: account.verified_at,
        }
    }
}

impl AccountRepository {
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl AccountPort for AccountRepository {
    async fn select_by_email(&self, self_email: String) -> anyhow::Result<Option<Account>> {
        let connection = &mut self.provider.get().await?;
        let result = accounts
            .select(AccountRow::as_select())
            .filter(email.eq(self_email))
            .first(connection)
            .await
            .optional()?
            .map(Account::from);

        Ok(result)
    }

    async fn insert(&self, account: Account) -> anyhow::Result<Account> {
        println!(
            "user:{};password_hash:{}",
            &account.email, &account.password_hash
        );

        let connection = &mut self.provider.get().await?;
        let result = diesel::insert_into(accounts)
            .values(AccountRow::from(account))
            .get_result::<AccountRow>(connection)
            .await
            .map(Account::from)?;

        Ok(result)
    }
}
