use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::adapter::{
    Services,
    config::ServerConfig,
    net::handlers::{
        AuthenticationHandler, OrganizationHandler,
        system::{AccountsHandler, IdentitiesHandler, OrganizationsHandler},
    },
};

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

    pub fn with_auth(mut self) -> Self {
        self.router = self.router.nest(
            "/api/v1/auth",
            AuthenticationHandler::v1(self.services.clone()),
        );

        self
    }

    pub fn with_scope(mut self) -> Self {
        self.router = self.router.nest(
            "/api/v1/organization",
            OrganizationHandler::v1(self.services.clone()),
        );

        self
    }

    pub fn with_v1(mut self) -> Self {
        let routes_v1 = Router::new()
            .nest("/accounts", AccountsHandler::v1(self.services.clone()))
            .nest(
                "/organizations",
                OrganizationsHandler::v1(self.services.clone()),
            )
            .nest("/identities", IdentitiesHandler::v1(self.services.clone()));

        self.router = self.router.nest("/api/v1/system", routes_v1);

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
