use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::adapter::{
    Services,
    net::handlers::{Params, internal_error},
};
use crate::domain::{
    applications::{CreateAccount, UpdateAccount},
    entities::Account,
};

pub struct AccountsHandler;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountRequest {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
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

impl AccountsHandler {
    async fn get(State(services): State<Arc<Services>>) -> impl IntoResponse {
        let accounts = match services.account().find_all().await {
            Ok(accounts) => accounts,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = accounts
            .into_iter()
            .map(AccountResponse::from)
            .collect::<Vec<AccountResponse>>();

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn get_by_id(
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

    async fn post(
        State(services): State<Arc<Services>>,
        Json(payload): Json<CreateAccountRequest>,
    ) -> impl IntoResponse {
        let password_hash = match services.actor().generate_hash(payload.password).await {
            Ok(password_hash) => password_hash,
            Err(error) => return internal_error(error).into_response(),
        };

        let account = match services
            .account()
            .create(CreateAccount {
                email: payload.email,
                password_hash,
                first_name: payload.first_name,
                last_name: payload.last_name,
            })
            .await
        {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = AccountResponse::from(account);

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn patch_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateAccountRequest>,
    ) -> impl IntoResponse {
        let account = match services
            .account()
            .update(
                id,
                UpdateAccount {
                    email: payload.email,
                    first_name: payload.first_name,
                    last_name: payload.last_name,
                },
            )
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

    async fn delete_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let account = services.account().delete(id).await;

        let result = account;

        match result {
            Ok(_) => (StatusCode::NO_CONTENT,).into_response(),
            Err(error) => internal_error(error).into_response(),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(Self::get).post(Self::post))
            .route(
                "/{id}",
                get(Self::get_by_id)
                    .patch(Self::patch_by_id)
                    .delete(Self::delete_by_id),
            )
            .with_state(services)
    }
}
