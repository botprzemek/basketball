mod groups;
mod members;
mod permissions;
mod roles;

use members::MembersHandler;
use roles::RolesHandler;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::adapter::Services;

use crate::adapter::net::AuthenticationActor;

pub struct OrganizationsHandler;

impl OrganizationsHandler {
    async fn organization_by_id(
        State(services): State<Arc<Services>>,
        actor: AuthenticationActor,
    ) -> Response {
        let organization = match services
            .organization()
            .find_by_id(actor.organization_id)
            .await
        {
            Ok(organization) => organization,
            Err(_) => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        match organization {
            Some(organization) => (StatusCode::OK, Json(organization)).into_response(),
            None => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        let resources = Router::new()
            .route("/", get(Self::organization_by_id))
            .with_state(services.clone())
            .nest("/members", MembersHandler::v1(services.clone()))
            // .nest("/groups", GroupsHandler::v1(services.clone()))
            // .nest("/permissions", GroupsHandler::v1(services.clone()));
            .nest("/roles", RolesHandler::v1(services.clone()));

        Router::new().nest("/{organization_id}", resources)
    }
}
