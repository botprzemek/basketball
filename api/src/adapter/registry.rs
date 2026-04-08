use crate::adapter::config::{CacheConfig, DatabaseConfig};
use crate::adapter::net::ResponseCache;
use crate::adapter::providers::{RedisProvider, ScyllaProvider};
use crate::adapter::repositories::AccountRepository;

pub struct Registry {
    pub cache: ResponseCache,
    pub account: AccountRepository,
}

impl Registry {
    pub async fn new(config: &(impl CacheConfig + DatabaseConfig)) -> anyhow::Result<Self> {
        let (redis, scylla) =
            tokio::try_join!(RedisProvider::new(config), ScyllaProvider::new(config))?;

        let account = AccountRepository::new(scylla.get()).await?;
        let cache = ResponseCache::new(redis.get(), config.get_cache_ttl());

        Ok(Self { account, cache })
    }
}
