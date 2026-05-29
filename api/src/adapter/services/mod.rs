mod actor;
mod password;
mod token;

use actor::ActorService;
use password::PasswordService;
use token::TokenService;

pub use token::AuthenticationState;

use crate::domain::applications::{
    AccountApplication, IdentityApplication, OrganizationApplication,
};
use crate::{
    adapter::{
        Registry,
        repositories::{
            AccountRepository, IdentityRepository, OrganizationRepository,
        },
    },
};

pub type AccountService = AccountApplication<AccountRepository>;
pub type IdentityService = IdentityApplication<IdentityRepository>;
pub type OrganizationService = OrganizationApplication<OrganizationRepository>;

pub struct Services {
    account: AccountService,
    organization: OrganizationService,
    identity: IdentityService,
    actor: ActorService,
    token: TokenService,
}

impl Services {
    pub fn new(registry: &Registry) -> Self {
        let password = PasswordService::new();
        
        let account = AccountApplication::new(registry.account_repository.clone());
        let organization = OrganizationApplication::new(registry.organization_repository.clone());
        let identity = IdentityApplication::new(registry.identity_repository.clone());

        let actor = ActorService::new(
            password.clone(),
            AccountApplication::new(registry.account_repository.clone()),
            OrganizationApplication::new(registry.organization_repository.clone()),
            IdentityApplication::new(registry.identity_repository.clone()),
            TokenService::new("basketball".to_string(), "secret".to_string()),
        );
        let token = TokenService::new("basketball".to_string(), "secret".to_string());

        Self {
            account,
            organization,
            identity,
            actor,
            token,
        }
    }

    pub fn account(&self) -> &AccountService {
        &self.account
    }

    pub fn organization(&self) -> &OrganizationService {
        &self.organization
    }

    pub fn identity(&self) -> &IdentityService {
        &self.identity
    }

    pub fn actor(&self) -> &ActorService {
        &self.actor
    }

    pub fn token(&self) -> &TokenService {
        &self.token
    }
}
