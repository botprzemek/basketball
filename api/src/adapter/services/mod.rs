mod authentication;
mod cookie;
mod password;
mod token;

use authentication::AuthenticationService;
use cookie::CookieService;
use password::PasswordService;
use token::TokenService;

use crate::adapter::{
    Registry,
    config::TokenConfig,
    repositories::{
        AccountRepository, IdentityRepository, MemberRepository, OrganizationRepository,
        RoleRepository,
    },
};
use crate::domain::applications::{
    AccountApplication, IdentityApplication, MemberApplication, OrganizationApplication,
    RoleApplication,
};

pub type AccountService = AccountApplication<AccountRepository>;
pub type OrganizationService = OrganizationApplication<OrganizationRepository>;
pub type IdentityService = IdentityApplication<IdentityRepository>;
pub type MemberService = MemberApplication<MemberRepository>;
pub type RoleService = RoleApplication<RoleRepository>;

pub struct Services {
    token: TokenService,
    auth: AuthenticationService,

    organization: OrganizationService,
    member: MemberService,
    role: RoleService,
}

impl Services {
    pub fn new(config: &impl TokenConfig, registry: &Registry) -> Self {
        let token = TokenService::new(config.token_issuer(), config.token_secret());
        let auth = AuthenticationService::new(
            CookieService::new(),
            TokenService::new(config.token_issuer(), config.token_secret()),
            PasswordService::new(),
            AccountApplication::new(registry.account.clone()),
            IdentityApplication::new(registry.identity.clone()),
        );

        let organization = OrganizationApplication::new(registry.organization.clone());
        let member = MemberApplication::new(registry.member.clone());
        let role = RoleApplication::new(registry.role.clone());

        Self {
            organization,
            member,
            role,
            auth,
            token,
        }
    }

    pub fn auth(&self) -> &AuthenticationService {
        &self.auth
    }

    pub fn token(&self) -> &TokenService {
        &self.token
    }

    pub fn organization(&self) -> &OrganizationService {
        &self.organization
    }

    pub fn member(&self) -> &MemberService {
        &self.member
    }

    pub fn role(&self) -> &RoleService {
        &self.role
    }
}
