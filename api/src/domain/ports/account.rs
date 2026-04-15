use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Account;

#[async_trait]
pub trait AccountPort: Clone + Send + Sync {
    async fn select(&self) -> anyhow::Result<Vec<Account>>;
    async fn select_by_self(&self, id: Uuid) -> anyhow::Result<Option<Account>>;
    async fn select_by_email(&self, email: String) -> anyhow::Result<Option<Account>>;
    async fn insert(&self, account: Account) -> anyhow::Result<Account>;
    async fn update(&self, account: Account) -> anyhow::Result<Account>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
