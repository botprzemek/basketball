use std::sync::Arc;

use crate::adapter::{
    config::DatabaseConfig,
    providers::PostgresProvider,
    repositories::{AccountRepository, MemberRepository, OrganizationRepository, RoleRepository},
};

pub struct Registry {
    pub account: AccountRepository,
    pub organization: OrganizationRepository,
    pub member: MemberRepository,
    pub role: RoleRepository,
}

impl Registry {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let provider = Arc::new(PostgresProvider::new(config).await?);

        Ok(Self {
            account: AccountRepository::new(provider.clone()),
            organization: OrganizationRepository::new(provider.clone()),
            member: MemberRepository::new(provider.clone()),
            role: RoleRepository::new(provider.clone()),
        })
    }
}
