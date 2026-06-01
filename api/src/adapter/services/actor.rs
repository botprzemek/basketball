use uuid::Uuid;

use crate::adapter::services::{
    AccountService, IdentityService, OrganizationService, PasswordService, TokenService,
    token::AuthenticationState,
};
use crate::domain::entities::{Actor, AuthenticatedActor, Identity, Organization};
use crate::domain::{applications::CreateAccount, entities::Account};

pub struct ActorService {
    password: PasswordService,
    account: AccountService,
    _organization: OrganizationService,
    identity: IdentityService,
    token: TokenService,
}

const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$jtZakqCGyhTTEEPAvX5wFA$Vg9HqwADg/5cxFyOLH7PtPoArGPTolQ/+ZvPzlC9Td0";

impl ActorService {
    pub fn new(
        password: PasswordService,
        account: AccountService,
        _organization: OrganizationService,
        identity: IdentityService,
        token: TokenService,
    ) -> Self {
        Self {
            password,
            account,
            _organization,
            identity,
            token,
        }
    }

    pub async fn generate_hash(&self, password: String) -> anyhow::Result<String> {
        self.password.generate(password).await
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> anyhow::Result<Account> {
        let password_hash = self.generate_hash(password).await?;
        let account = self
            .account
            .create(CreateAccount {
                email,
                password_hash,
                first_name,
                last_name,
            })
            .await?;

        Ok(account)
    }

    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> anyhow::Result<AuthenticationState> {
        let account = self.account.find_by_email(email).await?;

        let hash_to_verify = match &account {
            Some(account) => account.password_hash.clone(),
            None => DUMMY_HASH.to_string(),
        };

        let is_password_ok = self.password.verify(password, hash_to_verify).await.is_ok();

        match (account, is_password_ok) {
            (Some(account), true) => self.token.issue_identity_selection(account.id),
            _ => Err(anyhow::anyhow!("Invalid authentication credentials")),
        }
    }

    pub async fn context(&self, token: &str) -> anyhow::Result<Vec<(Identity, Organization)>> {
        match self.token.authenticate(token)? {
            Actor::Selection(actor) => self.identity.find_by_account(actor.account_id).await,
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn select(
        &self,
        token: &str,
        organization_id: Uuid,
    ) -> anyhow::Result<AuthenticationState> {
        match self.token.authenticate(token)? {
            Actor::Selection(actor) => {
                let identity = match self
                    .identity
                    .find_by_self(actor.account_id, organization_id)
                    .await?
                {
                    Some(identity) => identity,
                    None => return Err(anyhow::anyhow!("Identity not found")),
                };

                self.token
                    .issue_authentication(identity.account_id, identity.organization_id)
            }
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn current(&self, token: &str) -> anyhow::Result<AuthenticatedActor> {
        match self.token.authenticate(token)? {
            Actor::Authorized(actor) => Ok(actor),
            _ => Err(anyhow::anyhow!("Not logged in")),
        }
    }

    pub async fn refresh(&self, token: &str) -> anyhow::Result<AuthenticationState> {
        self.token.refresh(token)
    }
}
