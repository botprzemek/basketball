use std::sync::Arc;

use crate::adapter::{
    config::DatabaseConfig,
    providers::PostgresProvider,
    repositories::{AccountRepository, IdentityRepository, OrganizationRepository, RoleRepository},
};

pub struct Registry {
    pub account_repository: AccountRepository,
    pub organization_repository: OrganizationRepository,
    pub identity_repository: IdentityRepository,
    pub role_repository: RoleRepository,
}

impl Registry {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let provider = Arc::new(PostgresProvider::new(config).await?);

        Ok(Self {
            account_repository: AccountRepository::new(provider.clone()),
            organization_repository: OrganizationRepository::new(provider.clone()),
            identity_repository: IdentityRepository::new(provider.clone()),
            role_repository: RoleRepository::new(provider.clone()),
        })
    }
}
