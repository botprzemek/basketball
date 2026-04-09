use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt::Display;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapter::{
    Services,
};
use crate::domain::{
    entities::Account,
};

fn internal_error<E: Display>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

pub struct AuthHandler;

#[derive(serde::Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateAccountPayload {
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UpdateAccountPayload {
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            first_name: account.first_name,
            last_name: account.last_name,
            is_active: account.is_active,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

impl AuthHandler {
    async fn register(
        State(services): State<Arc<Services>>
    ) -> impl IntoResponse {
        (StatusCode::OK,).into_response()
    }

    async fn login(
        State(services): State<Arc<Services>>
    ) -> impl IntoResponse {
        (StatusCode::OK,).into_response()
    }
    
    async fn identify(
        State(services): State<Arc<Services>>
    ) -> impl IntoResponse {
        (StatusCode::OK,).into_response()
    }
    
    async fn refresh(
        State(services): State<Arc<Services>>
    ) -> impl IntoResponse {
        (StatusCode::OK,).into_response()
    }
    
    async fn logout(
        State(services): State<Arc<Services>>
    ) -> impl IntoResponse {
        (StatusCode::OK,).into_response()
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/register", post(AuthHandler::register))
            .route("/login", get(AuthHandler::login))
            .route("/identify", patch(AuthHandler::identify))
            .route("/refresh", delete(AuthHandler::refresh))
            .route("/logout", get(AuthHandler::logout))
            .with_state(services)
    }
}
