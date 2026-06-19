use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::adapter::Services;
use crate::adapter::net::{Actor, AuthenticationActor};

impl<S> FromRequestParts<S> for AuthenticationActor
where
    Arc<Services>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let services = Arc::<Services>::from_ref(state);
        let cookies = CookieJar::from_headers(&parts.headers);

        let token = match services.auth().get_access_token(&cookies) {
            Some(token) => token,
            None => return Err(services.auth().logout(StatusCode::UNAUTHORIZED)),
        };

        match services.token().authenticate(token) {
            Ok(Actor::Authorized(actor)) => Ok(actor),
            _ => Err(services.auth().logout(StatusCode::UNAUTHORIZED)),
        }
    }
}
