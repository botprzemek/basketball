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
    net::handlers::{Pagination, Params},
};
use crate::domain::{
    entities::Account,
};

fn internal_error<E: Display>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

pub struct AccountHandler;

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

impl AccountHandler {
    pub async fn get(
        State(services): State<Arc<Services>>,
        Query(Pagination { page, per_page}): Query<Pagination>
    ) -> impl IntoResponse {
        let accounts = match services.account().find_all(page, per_page).await {
            Ok(accounts) => accounts,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = accounts
            .into_iter()
            .map(AccountResponse::from)
            .collect::<Vec<AccountResponse>>();

        (StatusCode::OK, Json(result)).into_response()
    }

    pub async fn get_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let account = match services.account().find_by_id(id).await {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = account.map(AccountResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    pub async fn post(
        State(services): State<Arc<Services>>,
        Json(payload): Json<CreateAccountPayload>,
    ) -> impl IntoResponse {
        let account = match services
            .account()
            .create(payload.password, payload.first_name, payload.last_name)
            .await
        {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = AccountResponse::from(account);

        (StatusCode::CREATED, Json(result)).into_response()
    }

    pub async fn patch_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateAccountPayload>,
    ) -> impl IntoResponse {
        let account = match services
            .account()
            .update(id, payload.password, payload.first_name, payload.last_name)
            .await
        {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = account.map(AccountResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    pub async fn delete_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        match services.account().delete(id).await {
            Ok(_) => (StatusCode::NO_CONTENT,).into_response(),
            Err(error) => internal_error(error).into_response(),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(AccountHandler::get))
            .route("/", post(AccountHandler::post))
            .route("/{id}", get(AccountHandler::get_by_id))
            .route("/{id}", patch(AccountHandler::patch_by_id))
            .route("/{id}", delete(AccountHandler::delete_by_id))
            .with_state(services)
    }
}
