pub mod v1;

pub use v1::AuthenticationHandler;
pub use v1::OrganizationHandler;

pub use v1::system;

use axum::http::StatusCode;
use serde::Deserialize;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Deserialize, Default)]
pub struct Params {
    id: Uuid,
}

fn internal_error<E: Display>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
