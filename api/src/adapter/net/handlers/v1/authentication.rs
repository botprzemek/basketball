use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;
use std::sync::Arc;
use time::Duration;
use uuid::Uuid;

use crate::adapter::{Services, services::AuthenticationState};

pub struct AuthenticationHandler;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifyRequest {
    pub identity_id: Uuid,
}

impl AuthenticationHandler {
    fn create_auth_cookie<'a>(name: &'a str, value: String, max_age_seconds: i64) -> Cookie<'a> {
        Cookie::build((name, value))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(true)
            .max_age(Duration::seconds(max_age_seconds))
            .build()
    }

    fn remove_auth_cookie<'a>(name: &'a str) -> Cookie<'a> {
        Cookie::build((name, ""))
            .path("/")
            .http_only(true)
            .max_age(Duration::ZERO)
            .build()
    }

    fn invoke_identity<'a>(token: String) -> Cookie<'a> {
        Self::create_auth_cookie("identity-token", token, 60)
    }

    fn invoke_access_token<'a>(token: String) -> Cookie<'a> {
        Self::create_auth_cookie("access-token", token, 3600)
    }

    fn invoke_refresh_token<'a>(token: String) -> Cookie<'a> {
        Self::create_auth_cookie("refresh-token", token, 604800)
    }

    fn revoke_identity_token<'a>() -> Cookie<'a> {
        Self::remove_auth_cookie("identity-token")
    }

    fn revoke_access_token<'a>() -> Cookie<'a> {
        Self::remove_auth_cookie("access-token")
    }

    fn revoke_refresh_token<'a>() -> Cookie<'a> {
        Self::remove_auth_cookie("refresh-token")
    }

    fn revoke_all_auth_cookies(cookies: CookieJar) -> CookieJar {
        cookies
            .add(Self::revoke_identity_token())
            .add(Self::revoke_access_token())
            .add(Self::revoke_refresh_token())
    }

    fn handle_pending(cookies: CookieJar, state: AuthenticationState) -> (CookieJar, Response) {
        match state {
            AuthenticationState::Pending { identity } => (
                cookies
                    .add(Self::invoke_identity(identity))
                    .add(Self::revoke_access_token())
                    .add(Self::revoke_refresh_token()),
                StatusCode::OK.into_response(),
            ),
            AuthenticationState::Authenticated { .. } => {
                (cookies, StatusCode::UNAUTHORIZED.into_response())
            }
        }
    }

    fn handle_authenticated(
        cookies: CookieJar,
        state: AuthenticationState,
    ) -> (CookieJar, Response) {
        match state {
            AuthenticationState::Pending { .. } => (
                Self::revoke_all_auth_cookies(cookies),
                StatusCode::UNAUTHORIZED.into_response(),
            ),
            AuthenticationState::Authenticated { access, refresh } => (
                cookies
                    .add(Self::revoke_identity_token())
                    .add(Self::invoke_access_token(access))
                    .add(Self::invoke_refresh_token(refresh)),
                StatusCode::NO_CONTENT.into_response(),
            ),
        }
    }

    async fn register(
        State(services): State<Arc<Services>>,
        Json(RegisterRequest {
            email,
            password,
            first_name,
            last_name,
        }): Json<RegisterRequest>,
    ) -> impl IntoResponse {
        match services
            .actor()
            .register(email, password, first_name, last_name)
            .await
        {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn login(
        State(services): State<Arc<Services>>,
        cookies: CookieJar,
        Json(LoginRequest { email, password }): Json<LoginRequest>,
    ) -> (CookieJar, impl IntoResponse) {
        let state = match services.actor().authenticate(email, password).await {
            Ok(state) => state,
            Err(_) => return (cookies, StatusCode::UNAUTHORIZED.into_response()),
        };

        Self::handle_pending(cookies, state)
    }

    async fn me(State(services): State<Arc<Services>>, cookies: CookieJar) -> impl IntoResponse {
        let token = match cookies.get("identity-token") {
            Some(token) => token.value(),
            None => {
                return (
                    Self::revoke_all_auth_cookies(cookies),
                    StatusCode::UNAUTHORIZED.into_response(),
                );
            }
        };

        match services.actor().identities(token).await {
            Ok(identities) => (cookies, (StatusCode::OK, Json(identities)).into_response()),
            Err(_) => (
                Self::revoke_all_auth_cookies(cookies),
                StatusCode::UNAUTHORIZED.into_response(),
            ),
        }
    }

    async fn identify(
        State(services): State<Arc<Services>>,
        cookies: CookieJar,
        Json(payload): Json<IdentifyRequest>,
    ) -> impl IntoResponse {
        let token = match cookies.get("identity-token") {
            Some(token) => token.value(),
            None => {
                return (
                    Self::revoke_all_auth_cookies(cookies),
                    StatusCode::UNAUTHORIZED.into_response(),
                );
            }
        };

        let state = match services.actor().identify(payload.identity_id, token).await {
            Ok(state) => state,
            Err(_) => {
                return (
                    Self::revoke_all_auth_cookies(cookies),
                    StatusCode::UNAUTHORIZED.into_response(),
                );
            }
        };

        Self::handle_authenticated(cookies, state)
    }

    async fn refresh(
        State(services): State<Arc<Services>>,
        cookies: CookieJar,
    ) -> (CookieJar, impl IntoResponse) {
        let token = match cookies.get("refresh-token") {
            Some(token) => token.value(),
            None => {
                return (
                    Self::revoke_all_auth_cookies(cookies),
                    StatusCode::UNAUTHORIZED.into_response(),
                );
            }
        };

        let state = match services.actor().refresh(token).await {
            Ok(state) => state,
            Err(_) => {
                return (
                    Self::revoke_all_auth_cookies(cookies),
                    StatusCode::UNAUTHORIZED.into_response(),
                );
            }
        };

        Self::handle_authenticated(cookies, state)
    }

    async fn logout(cookies: CookieJar) -> (CookieJar, impl IntoResponse) {
        (
            Self::revoke_all_auth_cookies(cookies),
            StatusCode::NO_CONTENT,
        )
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/register", post(Self::register))
            .route("/login", post(Self::login))
            .route("/me", get(Self::me))
            .route("/identify", post(Self::identify))
            .route("/refresh", post(Self::refresh))
            .route("/logout", post(Self::logout))
            .with_state(services)
    }
}
