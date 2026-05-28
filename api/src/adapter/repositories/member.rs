use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{adapter::providers::PostgresProvider, domain::{entities::Member, ports::MemberPort}};

#[derive(Clone)]
pub struct MemberRepository {
    provider: Arc<PostgresProvider>,
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
    pub fn new(provider: Arc<PostgresProvider>) -> Self {
        Self {
            provider,
        }
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
