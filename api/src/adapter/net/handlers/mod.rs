pub mod v1;

pub use v1::AccountHandler;

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