use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::{Client, Row, Statement, types::Type};
use uuid::Uuid;

use crate::domain::{entities::Account, ports::AccountPort};

#[derive(Clone)]
pub struct AccountRepository {
    client: Arc<Client>,
    select: Statement,
    select_by_self: Statement,
    select_by_email: Statement,
    insert: Statement,
    update: Statement,
    delete: Statement,
}

impl From<&Row> for Account {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl AccountRepository {
    pub async fn new(client: Arc<Client>) -> anyhow::Result<Self> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS basketball.accounts (
                id UUID PRIMARY KEY,
                email STRING UNIQUE NOT NULL,
                password_hash STRING NOT NULL,
                first_name STRING NOT NULL,
                last_name STRING NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at TIMESTAMPTZ
            );

            CREATE INDEX IF NOT EXISTS idx_accounts_email ON basketball.accounts(email);
            CREATE INDEX IF NOT EXISTS idx_accounts_created_at ON basketball.accounts(created_at DESC);
        ")
            .await?;

        let select = client
            .prepare_typed(
                "
                SELECT
                    id,
                    email,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                FROM basketball.accounts
            ",
                &[],
            )
            .await?;

        let select_by_self = client
            .prepare_typed(
                "
                SELECT
                    id,
                    email,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                FROM basketball.accounts
                WHERE id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        let select_by_email = client
            .prepare_typed(
                "
                SELECT
                    id,
                    email,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    created_at,
                    updated_at
                FROM basketball.accounts
                WHERE email = $1
            ",
                &[Type::TEXT],
            )
            .await?;

        let insert = client
            .prepare_typed(
                "
                INSERT INTO basketball.accounts (
                    id,
                    email,
                    password_hash,
                    first_name,
                    last_name,
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
                    $6,
                    $7,
                    $8
                )
            ",
                &[
                    Type::UUID,
                    Type::TEXT,
                    Type::TEXT,
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
                UPDATE basketball.accounts
                SET (
                    email,
                    password_hash,
                    first_name,
                    last_name,
                    is_active,
                    updated_at
                ) = (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                )
            ",
                &[
                    Type::TEXT,
                    Type::TEXT,
                    Type::TEXT,
                    Type::TEXT,
                    Type::BOOL,
                    Type::TIMESTAMPTZ,
                ],
            )
            .await?;

        let delete = client
            .prepare_typed(
                "
                DELETE
                FROM basketball.accounts
                WHERE id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        Ok(Self {
            client,
            select,
            select_by_self,
            select_by_email,
            insert,
            update,
            delete,
        })
    }
}

#[async_trait]
impl AccountPort for AccountRepository {
    async fn select(&self) -> anyhow::Result<Vec<Account>> {
        let result = self
            .client
            .query(&self.select.clone(), &[])
            .await?
            .iter()
            .map(Account::from)
            .collect::<Vec<Account>>();

        Ok(result)
    }

    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        let result = self
            .client
            .query(&self.select_by_self.clone(), &[&id])
            .await?
            .first()
            .map(Account::from);

        Ok(result)
    }

    async fn select_by_email(&self, email: String) -> anyhow::Result<Option<Account>> {
        let result = self
            .client
            .query(&self.select_by_email.clone(), &[&email])
            .await?
            .first()
            .map(Account::from);

        Ok(result)
    }

    async fn insert(&self, account: Account) -> anyhow::Result<Account> {
        self.client
            .execute(
                &self.insert.clone(),
                &[
                    &account.id,
                    &account.email,
                    &account.password_hash,
                    &account.first_name,
                    &account.last_name,
                    &account.is_active,
                    &account.created_at,
                    &account.updated_at,
                ],
            )
            .await?;

        Ok(account)
    }

    async fn update(&self, account: Account) -> anyhow::Result<Account> {
        self.client
            .execute(
                &self.update.clone(),
                &[
                    &account.email,
                    &account.password_hash,
                    &account.first_name,
                    &account.last_name,
                    &account.is_active,
                    &account.updated_at,
                ],
            )
            .await?;

        Ok(account)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.client.execute(&self.delete.clone(), &[&id]).await?;

        Ok(())
    }
}
