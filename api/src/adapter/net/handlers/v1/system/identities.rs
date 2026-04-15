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
    applications::{CreateIdentity, UpdateIdentity},
    entities::Identity,
};

pub struct IdentitiesHandler;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIdentityRequest {
    pub organization_id: Uuid,
    pub account_id: Uuid,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIdentityRequest {
    pub organization_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IdentityResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<CreateIdentityRequest> for CreateIdentity {
    fn from(request: CreateIdentityRequest) -> Self {
        Self {
            organization_id: request.organization_id,
            account_id: request.account_id,
        }
    }
}

impl From<UpdateIdentityRequest> for UpdateIdentity {
    fn from(request: UpdateIdentityRequest) -> Self {
        Self {
            organization_id: request.organization_id,
            account_id: request.account_id,
        }
    }
}

impl From<Identity> for IdentityResponse {
    fn from(identity: Identity) -> Self {
        Self {
            id: identity.id,
            organization_id: identity.organization_id,
            account_id: identity.account_id,
            created_at: identity.created_at,
            updated_at: identity.updated_at,
        }
    }
}

impl IdentitiesHandler {
    async fn get(State(services): State<Arc<Services>>) -> impl IntoResponse {
        let identities = match services.identity().find_all().await {
            Ok(identities) => identities,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = identities
            .into_iter()
            .map(IdentityResponse::from)
            .collect::<Vec<IdentityResponse>>();

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn get_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let identity = match services.identity().find_by_identity(id).await {
            Ok(identity) => identity,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = identity.map(IdentityResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    async fn post(
        State(services): State<Arc<Services>>,
        Json(payload): Json<CreateIdentityRequest>,
    ) -> impl IntoResponse {
        let identity = match services.identity().create(payload.into()).await {
            Ok(identity) => identity,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = IdentityResponse::from(identity);

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn patch_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
        Json(payload): Json<UpdateIdentityRequest>,
    ) -> impl IntoResponse {
        let identity = match services.identity().update(id, payload.into()).await {
            Ok(identity) => identity,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = identity.map(IdentityResponse::from);

        match result {
            Some(result) => (StatusCode::OK, Json(result)).into_response(),
            None => (StatusCode::NOT_FOUND,).into_response(),
        }
    }

    async fn delete_by_id(
        State(services): State<Arc<Services>>,
        Path(Params { id }): Path<Params>,
    ) -> impl IntoResponse {
        let identity = services.identity().delete(id).await;

        let result = identity;

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
