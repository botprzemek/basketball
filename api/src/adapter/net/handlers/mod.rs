pub mod v1;

pub use v1::AuthenticationHandler;
pub use v1::OrganizationHandler;

use axum::http::StatusCode;
use std::fmt::Display;

fn internal_error<E: Display>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
