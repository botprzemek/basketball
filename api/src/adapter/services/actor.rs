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

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> anyhow::Result<AuthenticationState> {
        let account = self.account.find_by_email(email).await?;
        let hash_to_verify = match &account {
            Some(acc) => acc.password_hash.clone(),
            None => DUMMY_HASH.to_string(),
        };
        let is_password_ok = self.password.verify(password, hash_to_verify).await.is_ok();

        match (account, is_password_ok) {
            (Some(account), true) => self.token.issue_identity_selection(account.id),
            _ => Err(anyhow::anyhow!("Invalid authentication credentials")),
        }
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
