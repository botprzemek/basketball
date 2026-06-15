use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::adapter::services::{
    AccountService, CookieService, IdentityService, PasswordService, TokenService,
    token::AuthenticationState,
};

use crate::domain::{
    applications::CreateAccount,
    entities::Account,
    entities::{Actor, AuthenticatedActor, Identity, Organization},
};

pub struct AuthenticationService {
    cookie: CookieService,
    password: PasswordService,
    account: AccountService,
    identity: IdentityService,
    token: TokenService,
}

const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$jtZakqCGyhTTEEPAvX5wFA$Vg9HqwADg/5cxFyOLH7PtPoArGPTolQ/+ZvPzlC9Td0";

impl AuthenticationService {
    pub fn new(
        cookie: CookieService,
        token: TokenService,
        password: PasswordService,
        account: AccountService,
        identity: IdentityService,
    ) -> Self {
        Self {
            cookie,
            password,
            account,
            identity,
            token,
        }
    }

    pub fn get_identity_token(&self, cookies: &CookieJar) -> Option<String> {
        self.cookie.get_identity_token(cookies)
    }

    pub fn get_access_token(&self, cookies: &CookieJar) -> Option<String> {
        self.cookie.get_access_token(cookies)
    }

    pub fn get_refresh_token(&self, cookies: &CookieJar) -> Option<String> {
        self.cookie.get_refresh_token(cookies)
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> anyhow::Result<Account> {
        let password_hash = self.password.generate(password).await?;
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

    pub async fn context(&self, token: String) -> anyhow::Result<Vec<(Identity, Organization)>> {
        match self.token.authenticate(token)? {
            Actor::Selection(actor) => self.identity.find_by_account(actor.account_id).await,
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn select(
        &self,
        token: String,
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

    pub fn current(&self, token: String) -> anyhow::Result<AuthenticatedActor> {
        match self.token.authenticate(token)? {
            Actor::Authorized(actor) => Ok(actor),
            _ => Err(anyhow::anyhow!("Not logged in")),
        }
    }

    pub fn refresh(&self, token: String) -> anyhow::Result<AuthenticationState> {
        self.token.refresh(token)
    }

    pub fn logout(&self, status_code: StatusCode) -> Response {
        (self.cookie.revoke_all_auth_cookies(), status_code).into_response()
    }

    pub fn pending(&self, state: AuthenticationState) -> Response {
        match state {
            AuthenticationState::Authenticated { .. } => self.logout(StatusCode::UNAUTHORIZED),
            AuthenticationState::Pending { identity } => (
                CookieJar::new()
                    .add(self.cookie.invoke_identity(identity))
                    .add(self.cookie.revoke_access_token())
                    .add(self.cookie.revoke_refresh_token()),
                StatusCode::NO_CONTENT,
            )
                .into_response(),
        }
    }

    pub fn authenticate(&self, state: AuthenticationState) -> Response {
        match state {
            AuthenticationState::Pending { .. } => self.logout(StatusCode::UNAUTHORIZED),
            AuthenticationState::Authenticated { access, refresh } => (
                CookieJar::new()
                    .add(self.cookie.revoke_identity_token())
                    .add(self.cookie.invoke_access_token(access))
                    .add(self.cookie.invoke_refresh_token(refresh)),
                StatusCode::NO_CONTENT,
            )
                .into_response(),
        }
    }
}
