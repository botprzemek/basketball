use std::sync::Arc;

use crate::adapter::config::{CacheConfig, DatabaseConfig};
use crate::adapter::net::ResponseCache;
use crate::adapter::providers::{RedisProvider, ScyllaProvider};
use crate::adapter::repositories::AccountRepository;

pub struct Registry {
    cache: Arc<ResponseCache>,
    account: Arc<AccountRepository>,
}

impl Registry {
    pub async fn new(config: &(impl CacheConfig + DatabaseConfig)) -> anyhow::Result<Self> {
        let (redis, scylla) =
            tokio::try_join!(RedisProvider::new(config), ScyllaProvider::new(config))?;

        let account = Arc::new(AccountRepository::new(scylla.get()).await?);
        let cache = Arc::new(ResponseCache::new(redis.get(), config.get_cache_ttl()));

        Ok(Self { account, cache })
    }

    pub fn account(&self) -> Arc<AccountRepository> {
        self.account.clone()
    }

    pub fn cache(&self) -> Arc<ResponseCache> {
        self.cache.clone()
    }
}
