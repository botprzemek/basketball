use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::adapter::services::{
    AccountService, CookieService, MemberService, PasswordService, TokenService,
    token::AuthenticationState,
};

use crate::adapter::net::{Actor, AuthenticationActor};

use crate::domain::{
    applications::CreateAccount,
    entities::{Account, Member, Organization},
};

pub struct AuthenticationService {
    cookie: CookieService,
    token: TokenService,
    password: PasswordService,

    account: AccountService,
    member: MemberService,
}

const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$jtZakqCGyhTTEEPAvX5wFA$Vg9HqwADg/5cxFyOLH7PtPoArGPTolQ/+ZvPzlC9Td0";

impl AuthenticationService {
    pub fn new(
        cookie: CookieService,
        token: TokenService,
        password: PasswordService,
        account: AccountService,
        member: MemberService,
    ) -> Self {
        Self {
            cookie,
            password,
            token,
            account,
            member,
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

    pub async fn register(&self, email: String, password: String) -> anyhow::Result<Account> {
        let password_hash = self.password.generate(password).await?;
        let account = self
            .account
            .create(CreateAccount {
                email,
                password_hash,
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

    pub async fn context(&self, token: String) -> anyhow::Result<Vec<(Member, Organization)>> {
        match self.token.authenticate(token)? {
            Actor::Selected(selected_actor) => {
                self.member.find_by_account(selected_actor.account_id).await
            }
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub async fn select(
        &self,
        token: String,
        organization_id: Uuid,
    ) -> anyhow::Result<AuthenticationState> {
        match self.token.authenticate(token)? {
            Actor::Selected(selected_actor) => {
                let member = match self
                    .member
                    .find_by_self(organization_id, selected_actor.account_id)
                    .await?
                {
                    Some(member) => member,
                    None => return Err(anyhow::anyhow!("Identity not found")),
                };

                self.token
                    .issue_authentication(member.account_id, member.organization_id)
            }
            Actor::Authorized(_) => Err(anyhow::anyhow!("Already logged in")),
        }
    }

    pub fn current(&self, token: String) -> anyhow::Result<AuthenticationActor> {
        match self.token.authenticate(token)? {
            Actor::Authorized(autorized_actor) => Ok(autorized_actor),
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
