use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use scylla::{
    DeserializeRow, SerializeRow,
    client::session::Session,
    statement::prepared::PreparedStatement,
    value::CqlTimestamp,
};
use uuid::Uuid;

use crate::domain::entities::Account;
use crate::domain::ports::AccountPort;

#[derive(Clone)]
pub struct AccountRepository {
    session: Arc<Session>,
    select_all: PreparedStatement,
    select_by_id: PreparedStatement,
    insert: PreparedStatement,
    update: PreparedStatement,
    delete: PreparedStatement,
}

#[derive(SerializeRow, DeserializeRow)]
pub struct AccountRow {
    pub id: Uuid,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: Option<CqlTimestamp>,
}

impl From<AccountRow> for Account {
    fn from(row: AccountRow) -> Self {
        fn convert_timestamp(ms: i64) -> DateTime<Utc> {
            DateTime::from_timestamp_millis(ms)
                .unwrap_or(Utc::now())
                .with_timezone(&Utc)
        }

        Self {
            id: row.id,
            password_hash: row.password_hash,
            first_name: row.first_name,
            last_name: row.last_name,
            is_active: row.is_active,
            created_at: convert_timestamp(row.created_at.0),
            updated_at: row.updated_at.map(|d| convert_timestamp(d.0)),
        }
    }
}

impl From<Account> for AccountRow {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            password_hash: account.password_hash,
            first_name: account.first_name,
            last_name: account.last_name,
            is_active: account.is_active,
            created_at: CqlTimestamp(account.created_at.timestamp_millis()),
            updated_at: account
                .updated_at
                .map(|d| CqlTimestamp(d.timestamp_millis())),
        }
    }
}

impl AccountRepository {
    pub async fn new(session: Arc<Session>) -> anyhow::Result<Self> {
        session
            .query_unpaged(
                "
                CREATE KEYSPACE IF NOT EXISTS api
                WITH replication = {
                    'class': 'NetworkTopologyStrategy',
                    'replication_factor': 1
                }",
                (),
            )
            .await?;

        session
            .query_unpaged(
                "CREATE TABLE IF NOT EXISTS api.accounts (
                id uuid PRIMARY KEY,
                password_hash text,
                first_name text,
                last_name text,
                is_active boolean,
                created_at timestamp,
                updated_at timestamp
            )",
                (),
            )
            .await?;

        let select_all = session
            .prepare(
                "SELECT
                    id,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                FROM api.accounts",
            )
            .await?;

        let select_by_id = session
            .prepare(
                "SELECT
                    id,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                FROM api.accounts
                WHERE id = :id",
            )
            .await?;

        let insert = session
            .prepare(
                "INSERT
                INTO api.accounts (
                    id,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                )
                VALUES (
                    :id,
                    :password_hash,
                    :first_name,
                    :last_name,
                    :is_active,
                    :created_at,
                    :updated_at
                )
                IF NOT EXISTS",
            )
            .await?;

        let update = session
            .prepare(
                "UPDATE api.accounts 
                SET
                    password_hash = :password_hash,
                    first_name = :first_name, 
                    last_name = :last_name,
                    is_active = :is_active,
                    created_at = :created_at,
                    updated_at = :updated_at
                WHERE id = :id
                IF EXISTS",
            )
            .await?;

        let delete = session
            .prepare(
                "DELETE
                FROM api.accounts
                WHERE id = ?
                IF EXISTS",
            )
            .await?;

        Ok(Self {
            session,
            select_all,
            select_by_id,
            insert,
            update,
            delete,
        })
    }
}

#[async_trait]
impl AccountPort for AccountRepository {
    async fn select_all(&self, page_size: Option<usize>) -> anyhow::Result<Vec<Account>> {
        let mut statement = self.select_all.clone();
        let values = ();

        if let Some(page_size) = page_size {
            statement.set_page_size(page_size as i32);
        };

        let result = self
            .session
            .execute_iter(statement, &values)
            .await?
            .rows_stream::<AccountRow>()?
            .map_ok(Account::from)
            .try_collect::<Vec<Account>>()
            .await?;

        Ok(result)
    }

    async fn select_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        let values = (id,);

        let result = self
            .session
            .execute_unpaged(&self.select_by_id, &values)
            .await?
            .into_rows_result()?
            .maybe_first_row::<AccountRow>()?
            .map(Account::from);

        Ok(result)
    }

    async fn insert(&self, account: Account) -> anyhow::Result<Account> {
        let values = AccountRow::from(account);

        self.session.execute_unpaged(&self.insert, &values).await?;

        Ok(Account::from(values))
    }

    async fn update(&self, account: Account) -> anyhow::Result<Account> {
        let values = AccountRow::from(account);

        self.session.execute_unpaged(&self.update, &values).await?;

        Ok(Account::from(values))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        let values = (id,);

        self.session.execute_unpaged(&self.delete, &values).await?;

        Ok(())
    }
}
