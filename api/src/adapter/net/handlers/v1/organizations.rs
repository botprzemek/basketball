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

pub struct OrganizationsHandler;

#[derive(serde::Serialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateOrganizationPayload {
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UpdateOrganizationPayload {
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<Account> for OrganizationResponse {
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

impl OrganizationsHandler {
    async fn get(
        State(services): State<Arc<Services>>,
        Query(_pagination): Query<Pagination>
    ) -> impl IntoResponse {
        let accounts = match services.account().find_all().await {
            Ok(accounts) => accounts,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = accounts
            .into_iter()
            .map(OrganizationResponse::from)
            .collect::<Vec<OrganizationResponse>>();

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

        let result = account.map(OrganizationResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    async fn post(
        State(services): State<Arc<Services>>,
        Json(payload): Json<CreateOrganizationPayload>,
    ) -> impl IntoResponse {
        let account = match services
            .account()
            .create(payload.password, payload.first_name, payload.last_name)
            .await
        {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = OrganizationResponse::from(account);

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn patch_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateOrganizationPayload>,
    ) -> impl IntoResponse {
        let account = match services
            .account()
            .update(id, payload.password, payload.first_name, payload.last_name)
            .await
        {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = account.map(OrganizationResponse::from);

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
            .route("/", get(OrganizationsHandler::get))
            .route("/", post(OrganizationsHandler::post))
            .route("/{id}", get(OrganizationsHandler::get_by_id))
            .route("/{id}", patch(OrganizationsHandler::patch_by_id))
            .route("/{id}", delete(OrganizationsHandler::delete_by_id))
            .with_state(services)
    }
}
