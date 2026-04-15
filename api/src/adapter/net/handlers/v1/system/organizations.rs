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
    applications::{CreateOrganization, UpdateOrganization},
    entities::Organization,
};

pub struct OrganizationsHandler;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<CreateOrganizationRequest> for CreateOrganization {
    fn from(request: CreateOrganizationRequest) -> Self {
        Self {
            name: request.name,
            slug: request.slug,
        }
    }
}

impl From<UpdateOrganizationRequest> for UpdateOrganization {
    fn from(request: UpdateOrganizationRequest) -> Self {
        Self {
            name: request.name,
            slug: request.slug,
        }
    }
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            is_active: organization.is_active,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
        }
    }
}

impl OrganizationsHandler {
    async fn get(State(services): State<Arc<Services>>) -> impl IntoResponse {
        let organizations = match services.organization().find_all().await {
            Ok(organizations) => organizations,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = organizations
            .into_iter()
            .map(OrganizationResponse::from)
            .collect::<Vec<OrganizationResponse>>();

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn get_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let organization = match services.organization().find_by_id(id).await {
            Ok(organization) => organization,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = organization.map(OrganizationResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    async fn post(
        State(services): State<Arc<Services>>,
        Json(payload): Json<CreateOrganizationRequest>,
    ) -> impl IntoResponse {
        let organization = match services.organization().create(payload.into()).await {
            Ok(organization) => organization,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = OrganizationResponse::from(organization);

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn patch_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateOrganizationRequest>,
    ) -> impl IntoResponse {
        let organization = match services.organization().update(id, payload.into()).await {
            Ok(account) => account,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = organization.map(OrganizationResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    async fn delete_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let organization = services.organization().delete(id).await;

        let result = organization;

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
