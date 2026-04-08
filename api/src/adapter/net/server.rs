use axum::Router;
use axum::middleware::from_fn_with_state;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::adapter::Services;
use crate::adapter::config::ServerConfig;
use crate::adapter::net::handlers::AccountHandler;
use crate::adapter::net::middleware::cache_layer;

pub struct Gateway {
    address: String,
    router: Router,
    services: Arc<Services>,
}

impl Gateway {
    pub async fn new(config: &impl ServerConfig, services: Arc<Services>) -> anyhow::Result<Self> {
        let address = config.get_server_url();
        let router = Router::new();

        Ok(Gateway {
            address,
            router,
            services,
        })
    }

    pub fn with_v1(mut self) -> Self {
        let routes_v1 =
            Router::new().nest("/accounts", AccountHandler::v1(self.services.clone()));

        self.router = self.router.nest("/api/v1", routes_v1);

        self
    }

    pub fn with_cache(mut self) -> Self {
        let layer = from_fn_with_state(self.services.clone(), cache_layer);

        self.router = self.router.layer(layer);

        self
    }

    async fn graceful_shutdown() {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to install SIGNAL handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => { println!("\nGracefully exiting (CTRL+C)"); },
            _ = terminate => { println!("\nGracefully exiting (SIGTERM)"); },
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;

        println!("Listening on http://{}", self.address);
        axum::serve(listener, self.router.clone())
            .with_graceful_shutdown(Self::graceful_shutdown())
            .await?;

        Ok(())
    }
}
