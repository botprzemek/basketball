use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::PoolableConnection;

use crate::adapter::config::DatabaseConfig;

pub type ProviderConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct PostgresProvider {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl PostgresProvider {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let url = config.get_database_url();

        let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder()
            .max_size(10)
            .build(connection_manager)
            .await?;

        Ok(Self {
            pool,
        })
    }

    pub async fn get(&self) -> anyhow::Result<ProviderConnection<'static>> {
        self.pool.get().await
    }
}
