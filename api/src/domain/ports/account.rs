use async_trait::async_trait;

use crate::domain::entities::Account;

#[async_trait]
pub trait AccountPort: Clone + Send + Sync {
    async fn select_by_email(&self, email: String) -> anyhow::Result<Option<Account>>;
    async fn insert(&self, account: Account) -> anyhow::Result<Account>;
}
