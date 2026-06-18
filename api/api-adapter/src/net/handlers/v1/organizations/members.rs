use std::sync::Arc;

use axum::extract::Path;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use uuid::Uuid;

use crate::Services;
use crate::net::AuthenticatedActor;

pub struct MembersHandler;

impl MembersHandler {
    async fn members(
        State(services): State<Arc<Services>>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        match services.member().find_all(actor.organization_id).await {
            Ok(members) => (StatusCode::OK, Json(members)).into_response(),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    async fn member_by_id(
        State(services): State<Arc<Services>>,
        Path(id): Path<Uuid>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        match services
            .member()
            .find_by_identity(actor.organization_id, id)
            .await
        {
            Ok(members) => (StatusCode::OK, Json(members)).into_response(),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    async fn member_roles(
        State(services): State<Arc<Services>>,
        Path(id): Path<Uuid>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        match services
            .role()
            .find_by_identity(actor.organization_id, id)
            .await
        {
            Ok(members) => (StatusCode::OK, Json(members)).into_response(),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(Self::members))
            .route("/{id}", get(Self::member_by_id))
            .route("/{id}/roles", get(Self::member_roles))
            .with_state(services)
    }
}
