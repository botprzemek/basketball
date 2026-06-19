use anyhow::Context;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager};

use crate::adapter::config::DatabaseConfig;

pub type ProviderConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

pub struct PostgresProvider {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresProvider {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let url = config.database_url();
        let connection_manager =
            AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder()
            .max_size(10)
            .build(connection_manager)
            .await?;

        pool.get().await?;

        Ok(Self { pool })
    }

    pub async fn get(&self) -> anyhow::Result<ProviderConnection<'_>> {
        self.pool
            .get()
            .await
            .context("Failed to get connection from Postgres pool")
    }
}
