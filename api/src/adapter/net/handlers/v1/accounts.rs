use axum::{
    Json, Router,
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt::Display;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::applications::AccountApplication;
use crate::domain::entities::AccountPartial;
use crate::domain::ports::AccountPort;
use crate::{
    adapter::{net::handlers::Params, repositories::AccountRepository},
    domain::entities::Account,
};

fn internal_error<E: Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub struct AccountHandler;

#[derive(Clone)]
pub struct AccountState<P: AccountPort> {
    pub account_service: Arc<AccountApplication<P>>,
}

// TODO

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

impl From<UpdateAccountPayload> for AccountPartial {
    fn from(payload: UpdateAccountPayload) -> Self {
        Self {
            password: payload.password,
            first_name: payload.first_name,
            last_name: payload.last_name,
        }
    }
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

impl<P: AccountPort> FromRef<AccountState<P>> for Arc<AccountApplication<P>> {
    fn from_ref(state: &AccountState<P>) -> Self {
        state.account_service.clone()
    }
}

impl AccountHandler {
    pub async fn get<P>(
        State(account_service): State<Arc<AccountApplication<P>>>,
    ) -> anyhow::Result<impl IntoResponse, (StatusCode, String)>
    where
        P: AccountPort + Send + Sync + 'static,
    {
        let result = account_service.find_all().await.map_err(internal_error)?;

        Ok((StatusCode::OK, Json(result)).into_response())
    }

    pub async fn get_by_id<P>(
        State(account_service): State<Arc<AccountApplication<P>>>,
        Path(Params { id }): Path<Params>,
    ) -> anyhow::Result<impl IntoResponse, (StatusCode, String)>
    where
        P: AccountPort + Send + Sync + 'static,
    {
        let result = account_service
            .find_by_id(id)
            .await
            .map_err(internal_error)?;

        Ok((StatusCode::OK, Json(result)).into_response())
    }

    pub async fn post<P>(
        State(account_service): State<Arc<AccountApplication<P>>>,
        Json(payload): Json<CreateAccountPayload>,
    ) -> anyhow::Result<impl IntoResponse, (StatusCode, String)>
    where
        P: AccountPort + Send + Sync + 'static,
    {
        let result = account_service
            .create(payload.password, payload.first_name, payload.last_name)
            .await
            .map_err(internal_error)?;

        Ok((StatusCode::OK, Json(result)).into_response())
    }

    pub async fn patch_by_id<P>(
        State(account_service): State<Arc<AccountApplication<P>>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateAccountPayload>,
    ) -> anyhow::Result<impl IntoResponse, (StatusCode, String)>
    where
        P: AccountPort + Send + Sync + 'static,
    {
        let result = account_service
            .update(id, AccountPartial::from(payload))
            .await
            .map_err(internal_error)?;

        Ok((StatusCode::OK, Json(result)).into_response())
    }

    pub async fn delete_by_id<P>(
        State(account_service): State<Arc<AccountApplication<P>>>,
        Path(Params { id }): Path<Params>,
    ) -> anyhow::Result<impl IntoResponse, (StatusCode, String)>
    where
        P: AccountPort + Send + Sync + 'static,
    {
        account_service.delete(id).await.map_err(internal_error)?;

        Ok((StatusCode::NO_CONTENT,).into_response())
    }

    pub fn v1(account_service: Arc<AccountApplication<AccountRepository>>) -> Router {
        let state = AccountState { account_service };

        Router::new()
            .route("/", get(AccountHandler::get))
            .route("/", post(AccountHandler::post))
            .route("/{id}", get(AccountHandler::get_by_id))
            .route("/{id}", patch(AccountHandler::patch_by_id))
            .route("/{id}", delete(AccountHandler::delete_by_id))
            .with_state(state)
    }
}
