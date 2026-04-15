use std::sync::Arc;
use tokio_postgres::{Client, NoTls, connect};

use crate::adapter::config::DatabaseConfig;

pub struct PostgresProvider {
    client: Arc<Client>,
}

impl PostgresProvider {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let url = config.get_database_url();
        let (client, connection) = connect(&url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        client
            .batch_execute(
                "
            DROP SCHEMA IF EXISTS basketball CASCADE;
            CREATE SCHEMA IF NOT EXISTS basketball;
        ",
            )
            .await?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn get(&self) -> Arc<Client> {
        self.client.clone()
    }
}
