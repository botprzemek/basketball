use axum::{
    Router,
    http::{Method, header},
    middleware,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::adapter::{
    Services,
    config::ServerConfig,
    net::handlers::{AuthenticationHandler, OrganizationsHandler},
    net::middleware::context_middleware,
};

pub struct Gateway {
    address: String,
    router: Router,
    services: Arc<Services>,
}

impl Gateway {
    pub async fn new(config: &impl ServerConfig, services: Arc<Services>) -> anyhow::Result<Self> {
        let address = config.server_url();
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

    pub fn with_organizations(mut self) -> Self {
        let middleware = middleware::from_fn_with_state(self.services.clone(), context_middleware);

        self.router = self.router.nest(
            "/api/v1/organizations",
            OrganizationsHandler::v1(self.services.clone()).layer(middleware),
        );

        self
    }

    pub fn with_cors(mut self) -> Self {
        // TODO - Cors settings
        let cors = CorsLayer::new()
            .allow_origin(
                "http://localhost:3001"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            )
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_credentials(true);

        self.router = self.router.layer(cors);

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
