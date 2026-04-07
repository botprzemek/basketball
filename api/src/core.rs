use std::sync::Arc;

use crate::adapter::{Config, Gateway, Registry, Services};

pub struct Core {
    gateway: Arc<Gateway>,
}

pub struct CoreBuilder {
    config: Arc<Config>,
    registry: Option<Arc<Registry>>,
    services: Option<Arc<Services>>,
    gateway: Option<Arc<Gateway>>,
}

impl Core {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.gateway.run().await
    }
}

impl CoreBuilder {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;

        Ok(Self {
            config: Arc::new(config),
            registry: None,
            services: None,
            gateway: None,
        })
    }

    pub async fn registry(&mut self) -> anyhow::Result<&mut Self> {
        let registry = Registry::new(self.config.as_ref()).await?;

        self.registry = Some(Arc::new(registry));

        Ok(self)
    }

    pub async fn services(&mut self) -> anyhow::Result<&mut Self> {
        let registry = self
            .registry
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Registry not initialized"))?;
        let services = Services::new(registry);

        self.services = Some(Arc::new(services));

        Ok(self)
    }

    pub async fn gateway(&mut self) -> anyhow::Result<&mut Self> {
        let services = self
            .services
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Services not initialized"))?;
        let gateway = Gateway::new(self.config.as_ref(), services)
            .await?
            .with_v1();
        // .with_cache();

        self.gateway = Some(Arc::new(gateway));

        Ok(self)
    }

    pub fn build(&mut self) -> anyhow::Result<Core> {
        let gateway = self
            .gateway
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Gateway not initialized"))?;

        Ok(Core { gateway })
    }
}
