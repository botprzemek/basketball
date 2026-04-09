use async_trait::async_trait;

#[async_trait]
pub trait PasswordPort: Clone + Send + Sync {
    fn new() -> Self;
    async fn generate(&self, password: String) -> anyhow::Result<String>;
    async fn verify(&self, password: String, password_hash: String) -> anyhow::Result<()>;
}
