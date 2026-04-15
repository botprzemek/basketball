use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::{Client, Row, Statement, types::Type};
use uuid::Uuid;

use crate::domain::{entities::Member, ports::MemberPort};

#[derive(Clone)]
pub struct MemberRepository {
    client: Arc<Client>,
    select: Statement,
    select_by_organization: Statement,
    // TODO
    // BATCH TRANSACTION insert: Statement,
}

impl From<&Row> for Member {
    fn from(row: &Row) -> Self {
        Self {
            identity_id: row.get("identity_id"),
            account_id: row.get("account_id"),
            organization_id: row.get("organization_id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl MemberRepository {
    pub async fn new(client: Arc<Client>) -> anyhow::Result<Self> {
        client
            .batch_execute(
                "
            CREATE OR REPLACE VIEW basketball.members AS
            SELECT
                basketball.identities.id AS identity_id,
                basketball.identities.account_id AS account_id,
                basketball.identities.organization_id AS organization_id,
                basketball.accounts.email AS email,
                basketball.accounts.first_name AS first_name,
                basketball.accounts.last_name AS last_name,
                basketball.identities.created_at AS created_at,
                basketball.identities.updated_at AS updated_at
            FROM
                basketball.identities,
                basketball.accounts
            WHERE basketball.identities.account_id = basketball.accounts.id;
        ",
            )
            .await?;

        let select = client
            .prepare_typed(
                "
                SELECT
                    identity_id,
                    account_id,
                    organization_id,
                    email,
                    first_name,
                    last_name,
                    created_at,
                    updated_at
                FROM basketball.members
            ",
                &[],
            )
            .await?;

        let select_by_organization = client
            .prepare_typed(
                "
                SELECT
                    identity_id,
                    account_id,
                    organization_id,
                    email,
                    first_name,
                    last_name,
                    created_at,
                    updated_at
                FROM basketball.members
                WHERE organization_id = $1
            ",
                &[Type::UUID],
            )
            .await?;

        Ok(Self {
            client,
            select,
            select_by_organization,
        })
    }
}

#[async_trait]
impl MemberPort for MemberRepository {
    async fn select(&self) -> anyhow::Result<Vec<Member>> {
        let result = self
            .client
            .query(&self.select.clone(), &[])
            .await?
            .iter()
            .map(Member::from)
            .collect::<Vec<Member>>();

        Ok(result)
    }

    async fn select_by_organization(&self, organization_id: Uuid) -> anyhow::Result<Vec<Member>> {
        let result = self
            .client
            .query(&self.select_by_organization.clone(), &[&organization_id])
            .await?
            .iter()
            .map(Member::from)
            .collect::<Vec<Member>>();

        Ok(result)
    }
}
