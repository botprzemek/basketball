pub mod v1;

pub use v1::AccountHandler;
pub use v1::AuthHandler;
pub use v1::OrganizationsHandler;
pub use v1::IdentitiesHandler;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Default)]
pub struct Params {
    id: Uuid,
}

#[derive(Deserialize, Default)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}