use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use std::sync::Arc;

use crate::Services;
use crate::net::AuthenticatedActor;

pub struct RolesHandler;

impl RolesHandler {
    async fn roles(
        State(services): State<Arc<Services>>,
        actor: AuthenticatedActor,
    ) -> impl IntoResponse {
        match services.role().find(actor.organization_id).await {
            Ok(roles) => (StatusCode::OK, Json(roles)).into_response(),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(Self::roles))
            .with_state(services)
    }
}
