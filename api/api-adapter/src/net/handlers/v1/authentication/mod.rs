mod context;

use context::ContextHandler;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::Services;

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

impl AuthenticationHandler {
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
            .auth()
            .register(email, password, first_name, last_name)
            .await
        {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn login(
        State(services): State<Arc<Services>>,
        Json(LoginRequest { email, password }): Json<LoginRequest>,
    ) -> Response {
        match services.auth().login(email, password).await {
            Ok(state) => services.auth().pending(state),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    async fn refresh(State(services): State<Arc<Services>>, cookies: CookieJar) -> Response {
        let token = match services.auth().get_refresh_token(&cookies) {
            Some(token) => token,
            None => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        match services.auth().refresh(token) {
            Ok(state) => services.auth().authenticate(state),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    async fn logout(State(services): State<Arc<Services>>) -> Response {
        services.auth().logout(StatusCode::NO_CONTENT)
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/register", post(Self::register))
            .route("/login", post(Self::login))
            .route("/refresh", post(Self::refresh))
            .route("/logout", post(Self::logout))
            .with_state(services.clone())
            .nest("/context", ContextHandler::v1(services.clone()))
    }
}
