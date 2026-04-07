use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use scylla::client::session::Session;
use scylla::statement::prepared::PreparedStatement;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
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

impl AccountRepository {
    pub async fn new(session: Arc<Session>) -> anyhow::Result<Self> {
        session
            .query_unpaged("DROP TABLE IF EXISTS api.accounts", ())
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

        let first_names = [
            "Jan", "Anna", "Piotr", "Maria", "Marek", "Ewa", "Adam", "Olga", "Jacek", "Iga",
            "Kamil", "Marta", "Leon", "Sara", "Hugo", "Nina",
        ];
        let last_names = [
            "Nowak",
            "Kowalski",
            "Wiśniewski",
            "Wójcik",
            "Kowalczyk",
            "Kamiński",
            "Zieliński",
            "Szymański",
            "Woźniak",
            "Dąbrowski",
            "Kozłowski",
            "Mazur",
            "Kwiatkowski",
            "Krawczyk",
            "Kaczmarek",
            "Zając",
        ];

        let insert_stmt = session.prepare(
            "INSERT INTO api.accounts (id, password_hash, first_name, last_name, is_active, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, toTimestamp(now()), NULL)"
        ).await?;

        for i in 0..16 {
            let id = uuid::Uuid::new_v4();
            let first_name = first_names[i];
            let last_name = last_names[i % last_names.len()];
            let hash = "$argon2id$v=19$m=19456,t=2,p=1$c2FsdHNhbHQ$Wv6...";

            session
                .execute_unpaged(&insert_stmt, (id, hash, first_name, last_name, true))
                .await?;
        }

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

impl From<AccountRow> for Account {
    fn from(row: AccountRow) -> Self {
        fn convert_timestamp(ms: i64) -> DateTime<Utc> {
            DateTime::from_timestamp_millis(ms)
                .unwrap()
                .with_timezone(&Utc)
        }

        Self {
            id: row.id,
            password_hash: row.password_hash,
            first_name: row.first_name,
            last_name: row.last_name,
            is_active: row.is_active,
            created_at: convert_timestamp(row.created_at.0),
            updated_at: row.updated_at.map(|t| convert_timestamp(t.0)),
        }
    }
}

#[async_trait]
impl AccountPort for AccountRepository {
    async fn select_all(&self) -> anyhow::Result<Vec<Account>> {
        let result = self
            .session
            .execute_iter(self.select_all.clone(), &[])
            .await?
            .rows_stream::<AccountRow>()?
            .map_ok(Account::from)
            .try_collect::<Vec<Account>>()
            .await?;

        Ok(result)
    }

    async fn select_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        let result = self
            .session
            .execute_unpaged(&self.select_by_id.clone(), (id,))
            .await?
            .into_rows_result()?
            .maybe_first_row::<AccountRow>()?
            .map(Account::from);

        Ok(result)
    }

    async fn insert(&self, account: Account) -> anyhow::Result<Account> {
        self.session
            .execute_unpaged(&self.insert.clone(), &account)
            .await?;

        Ok(account)
    }

    async fn update(&self, account: Account) -> anyhow::Result<Account> {
        self.session
            .execute_unpaged(&self.update.clone(), &account)
            .await?;

        Ok(account)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.session
            .execute_unpaged(&self.delete.clone(), (id,))
            .await?;

        Ok(())
    }
}
