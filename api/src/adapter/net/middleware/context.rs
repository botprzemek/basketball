use crate::adapter::Services;
use crate::domain::entities::AuthenticatedActor;
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct ContextParams {
    organization_id: Uuid,
}

pub async fn context_middleware(
    State(services): State<Arc<Services>>,
    actor: AuthenticatedActor,
    Path(params): Path<ContextParams>,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    println!("GET / {}", &request.uri());

    if actor.organization_id != params.organization_id {
        return Err(services.auth().logout(StatusCode::UNAUTHORIZED));
    }

    Ok(next.run(request).await)
}
