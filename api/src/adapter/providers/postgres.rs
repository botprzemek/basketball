use crate::adapter::config::DatabaseConfig;

use std::sync::Arc;

use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;

pub struct PostgresProvider {
    client: Arc<Session>,
}

impl PostgresProvider {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let database_url = config.get_database_url();

        let session_builder = SessionBuilder::new()
            .known_node(database_url)
            .build()
            .await?;

        let client = Arc::new(session_builder);

        Ok(Self { client })
    }

    pub fn get(&self) -> Arc<Session> {
        self.client.clone()
    }
}
