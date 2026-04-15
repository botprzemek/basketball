use uuid::Uuid;

use crate::adapter::services::{
    AccountService, IdentityService, OrganizationService, PasswordService, TokenService,
    token::AuthenticationState,
};
use crate::domain::entities::{AccountIdentity, Actor};
use crate::domain::{applications::CreateAccount, entities::Account};

pub struct ActorService {
    password: PasswordService,
    account: AccountService,
    _organization: OrganizationService,
    identity: IdentityService,
    token: TokenService,
}

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

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> anyhow::Result<AuthenticationState> {
        let account = match self.account.find_by_email(email).await? {
            Some(account) => account,
            None => return Err(anyhow::anyhow!("Invalid credentials")),
        };

        self.password
            .verify(password, account.password_hash.clone())
            .await?;

        self.token.issue_identity_selection(account.id)
    }

    pub async fn identities(&self, token: &str) -> anyhow::Result<Vec<AccountIdentity>> {
        match self.token.authenticate(token)? {
            Actor::Selection(actor) => self.identity.find_by_account(actor.account_id).await,
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn identify(
        &self,
        selected_identity: Uuid,
        token: &str,
    ) -> anyhow::Result<AuthenticationState> {
        match self.token.authenticate(token)? {
            Actor::Selection(actor) => {
                let account_identity = match self
                    .identity
                    .find_by_account_identity(actor.account_id, selected_identity)
                    .await?
                {
                    Some(account_identity) => account_identity,
                    None => return Err(anyhow::anyhow!("Not valid")),
                };

                Ok(self.token.issue_authentication(
                    actor.account_id,
                    account_identity.identity_id,
                    account_identity.organization_id,
                )?)
            }
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn refresh(&self, token: &str) -> anyhow::Result<AuthenticationState> {
        self.token.refresh(token)
    }
}
