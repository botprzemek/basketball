use std::sync::Arc;

use crate::adapter::{Config, Gateway, Registry, Services};

pub struct Core {
    gateway: Gateway,
}

pub struct CoreBuilder {
    config: Arc<Config>,
    registry: Option<Registry>,
    services: Option<Arc<Services>>,
    gateway: Option<Gateway>,
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

        self.registry = Some(registry);

        Ok(self)
    }

    pub async fn services(&mut self) -> anyhow::Result<&mut Self> {
        let registry = self
            .registry
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Registry not initialized"))?;
        let services = Services::new(registry);
        use crate::domain::applications::{CreateAccount, CreateIdentity, CreateOrganization};

        let account = services
            .account()
            .create(CreateAccount {
                email: "v@nightcity.net".to_string(),
                password_hash: services
                    .actor()
                    .generate_hash("v-1S-th3-Be$t".to_string())
                    .await?,
                first_name: "Vincent".to_string(),
                last_name: "Wilson".to_string(),
            })
            .await?;

        let organization_1 = services
            .organization()
            .create(CreateOrganization {
                name: "Arasaka Corporation".to_string(),
                slug: "arasaka".to_string(),
            })
            .await?;

        let organization_2 = services
            .organization()
            .create(CreateOrganization {
                name: "Militech International Armaments".to_string(),
                slug: "militech".to_string(),
            })
            .await?;

        services
            .identity()
            .create(CreateIdentity {
                organization_id: organization_1.id,
                account_id: account.id,
            })
            .await?;

        services
            .identity()
            .create(CreateIdentity {
                organization_id: organization_2.id,
                account_id: account.id,
            })
            .await?;

        self.services = Some(Arc::new(services));

        Ok(self)
    }

    pub async fn gateway(&mut self) -> anyhow::Result<&mut Self> {
        let services = self
            .services
            .take()
            .ok_or_else(|| anyhow::anyhow!("Services not initialized"))?;

        let gateway = Gateway::new(self.config.as_ref(), services)
            .await?
            .with_auth()
            .with_scope()
            .with_v1();

        self.gateway = Some(gateway);

        Ok(self)
    }

    pub fn build(&mut self) -> anyhow::Result<Core> {
        let gateway = self
            .gateway
            .take()
            .ok_or_else(|| anyhow::anyhow!("Gateway not initialized"))?;

        Ok(Core { gateway })
    }
}
