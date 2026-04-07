use redis::aio::MultiplexedConnection;

use crate::adapter::config::CacheConfig;

pub struct RedisProvider {
    connection: MultiplexedConnection,
}

impl RedisProvider {
    pub async fn new(config: &impl CacheConfig) -> anyhow::Result<Self> {
        let cache_url = config.get_cache_url();
        let client = redis::Client::open(cache_url)?;

        let connection = client.get_multiplexed_async_connection().await?;

        Ok(Self { connection })
    }

    pub fn get(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}
