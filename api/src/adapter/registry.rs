use std::sync::Arc;

use crate::adapter::{
    config::DatabaseConfig,
    providers::PostgresProvider,
    repositories::{
        AccountRepository, OrganizationRepository, IdentityRepository, MemberRepository
    },
};

pub struct Registry {
    pub account_repository: AccountRepository,
    pub organization_repository: OrganizationRepository,
    pub identity_repository: IdentityRepository,
    pub member_repository: MemberRepository,
}

impl Registry {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let provider = Arc::new(PostgresProvider::new(config).await?);

        Ok(Self {
            account_repository: AccountRepository::new(provider),
            organization_repository: OrganizationRepository::new(provider),
            identity_repository: IdentityRepository::new(provider),
            member_repository: MemberRepository::new(provider),
        })
    }
}
