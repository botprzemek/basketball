use axum::response::Redirect;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapter::{Services, net::handlers::internal_error};
use crate::domain::entities::{AuthenticatedActor, Member, Organization};

pub struct OrganizationHandler;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub joined_at: DateTime<Utc>,
}

impl From<Member> for MembersResponse {
    fn from(member: Member) -> Self {
        Self {
            id: member.identity_id,
            email: member.email,
            first_name: member.first_name,
            last_name: member.last_name,
            joined_at: member.created_at,
        }
    }
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
        }
    }
}

impl OrganizationHandler {
    async fn organization(
        State(services): State<Arc<Services>>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        let organization = match services
            .organization()
            .find_by_id(actor.organization_id)
            .await
        {
            Ok(organization) => organization,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = match organization {
            Some(organization) => OrganizationResponse::from(organization),
            None => return Redirect::permanent("/api/v1/auth/logout").into_response(),
        };

        (StatusCode::OK, Json(result)).into_response()
    }

    async fn members(
        State(services): State<Arc<Services>>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        let members = match services
            .member()
            .find_by_organization(actor.organization_id)
            .await
        {
            Ok(members) => members,
            Err(error) => return internal_error(error).into_response(),
        };

        let result = members
            .into_iter()
            .map(MembersResponse::from)
            .collect::<Vec<MembersResponse>>();

        (StatusCode::OK, Json(result)).into_response()
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(Self::organization))
            .route("/members", get(Self::members))
            .with_state(services)
    }
}
