use axum::extract::Path;
use axum::response::Redirect;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use std::sync::Arc;
use uuid::Uuid;

use crate::adapter::{Services, net::handlers::internal_error};
use crate::domain::entities::AuthenticatedActor;

pub struct OrganizationHandler;

impl OrganizationHandler {
    async fn organizations_by_id(
        State(services): State<Arc<Services>>,
        Path(id): Path<Uuid>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        if id != actor.organization_id {
            return Redirect::permanent("/api/v1/auth/logout").into_response();
        }

        let organization = match services
            .organization()
            .find_by_id(actor.organization_id)
            .await
        {
            Ok(organization) => organization,
            Err(error) => return internal_error(error).into_response(),
        };

        match organization {
            Some(organization) => (StatusCode::OK, Json(organization)).into_response(),
            None => Redirect::permanent("/api/v1/auth/logout").into_response(),
        }
    }

    async fn roles(
        State(services): State<Arc<Services>>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        match services
            .role()
            .find_by_organization(actor.organization_id)
            .await
        {
            Ok(roles) => (StatusCode::OK, Json(roles)).into_response(),
            Err(error) => internal_error(error).into_response(),
        }
    }

    // async fn members(
    //     State(services): State<Arc<Services>>,
    //     actor: AuthenticatedActor,
    // ) -> impl IntoResponse {
    //     let members = match services
    //         .identity()
    //         .find_by_organization(actor.organization_id)
    //         .await
    //     {
    //         Ok(members) => members,
    //         Err(error) => return internal_error(error).into_response(),
    //     };

    //     let result = members
    //         .into_iter()
    //         .map(MembersResponse::from)
    //         .collect::<Vec<MembersResponse>>();

    //     (StatusCode::OK, Json(result)).into_response()
    // }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/{id}", get(Self::organizations_by_id))
            .route("/{id}/roles", get(Self::roles))
            // .route("/{id}/members", get(Self::members))
            .with_state(services)
    }
}
