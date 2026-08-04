use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapter::Services;

use crate::domain::entities::{Organization, Member};

pub struct ContextHandler;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextSelectRequest {
    pub organization: Organization,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextResponse {
    pub organization: Organization,
    pub member: Member,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextCurrentResponse {
    pub account_id: Uuid,
    pub organization_id: Uuid,
}

impl ContextHandler {
    async fn context(
        State(services): State<Arc<Services>>,
        cookies: CookieJar,
    ) -> impl IntoResponse {
        let token = match services.auth().get_identity_token(&cookies) {
            Some(token) => token,
            None => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        let organizations = match services.auth().context(token).await {
            Ok(organizations) => organizations,
            Err(_) => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        let organizations = organizations
            .into_iter()
            .map(|(member, organization)| ContextResponse {
                organization,
                member,
            })
            .collect::<Vec<_>>();

        (cookies, (StatusCode::OK, Json(organizations))).into_response()
    }

    async fn select(
        State(services): State<Arc<Services>>,
        cookies: CookieJar,
        Json(payload): Json<ContextSelectRequest>,
    ) -> impl IntoResponse {
        let token = match services.auth().get_identity_token(&cookies) {
            Some(token) => token,
            None => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        match services.auth().select(token, payload.organization.id).await {
            Ok(state) => services.auth().authenticate(state),
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    async fn current(State(services): State<Arc<Services>>, cookies: CookieJar) -> Response {
        let token = match services.auth().get_access_token(&cookies) {
            Some(token) => token,
            None => return services.auth().logout(StatusCode::UNAUTHORIZED),
        };

        match services.auth().current(token) {
            Ok(actor) => {
                let context = ContextCurrentResponse {
                    account_id: actor.account_id,
                    organization_id: actor.organization_id,
                };

                (cookies, (StatusCode::OK, Json(context))).into_response()
            }
            Err(_) => services.auth().logout(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn v1(services: Arc<Services>) -> Router {
        Router::new()
            .route("/", get(Self::context))
            .route("/select", post(Self::select))
            .route("/current", get(Self::current))
            .with_state(services)
    }
}
