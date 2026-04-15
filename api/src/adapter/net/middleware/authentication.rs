use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;

use crate::{
    adapter::Services,
    domain::entities::{Actor, AuthenticatedActor},
};

impl<S> FromRequestParts<S> for AuthenticatedActor
where
    Arc<Services>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let services = Arc::<Services>::from_ref(state);
        let cookies = CookieJar::from_headers(&parts.headers);

        let token = match cookies.get("access-token") {
            Some(token) => token.value(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        match services.token().authenticate(token) {
            Ok(Actor::Authorized(actor)) => Ok(actor),
            Ok(Actor::Selection(_)) => Err(StatusCode::UNAUTHORIZED),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
