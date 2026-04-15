use crate::adapter::{
    config::DatabaseConfig,
    providers::PostgresProvider,
    repositories::{
        AccountRepository, IdentityRepository, MemberRepository, OrganizationRepository,
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
        let postgres = PostgresProvider::new(config).await?;

        let (account_repository, organization_repository, identity_repository, member_repository) =
            tokio::try_join!(
                AccountRepository::new(postgres.get()),
                OrganizationRepository::new(postgres.get()),
                IdentityRepository::new(postgres.get()),
                MemberRepository::new(postgres.get())
            )?;

        Ok(Self {
            account_repository,
            organization_repository,
            identity_repository,
            member_repository,
        })
    }
}
