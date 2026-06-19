use anyhow::Context;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};

use rustls::RootCertStore;
use std::fs;
use std::io::BufReader;
use tokio_postgres::Config;
use tokio_postgres::config::SslMode;
use tokio_postgres_rustls_improved::MakeRustlsConnect;

use crate::adapter::config::DatabaseConfig;

pub type ProviderConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

pub struct PostgresProvider {
    pool: Pool<AsyncPgConnection>,
}

use diesel::ConnectionResult;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;

fn connect(_config: &str) -> BoxFuture<'_, ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        let ca_file = fs::read("/workspaces/auth/cockroach/certs/ca.crt").unwrap();
        let ca_certs = rustls_pemfile::certs(&mut BufReader::new(&ca_file[..]))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let cert_file = fs::read("/workspaces/auth/cockroach/certs/client.dev.crt").unwrap();
        let key_file = fs::read("/workspaces/auth/cockroach/certs/client.dev.key").unwrap();

        let client_certs = rustls_pemfile::certs(&mut BufReader::new(&cert_file[..]))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let client_key = rustls_pemfile::private_key(&mut BufReader::new(&key_file[..])).unwrap();

        let mut roots = RootCertStore::empty();
        for cert in ca_certs {
            roots.add(cert).unwrap();
        }

        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_client_auth_cert(client_certs, client_key.unwrap())
            .expect("build rustls client config");

        let tls = MakeRustlsConnect::new(tls_config);

        let mut pg_config = Config::new();
        pg_config
            .host("auth-database-1")
            .port(26257)
            .dbname("dev")
            .user("dev")
            .password("your-password")
            .ssl_mode(SslMode::Require);

        let (client, conn) = pg_config.connect(tls).await.expect("connect");

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("TLS error: {}", e);
            }
        });

        AsyncPgConnection::try_from(client).await
    };

    fut.boxed()
}

impl PostgresProvider {
    pub async fn new(config: &impl DatabaseConfig) -> anyhow::Result<Self> {
        let url = config.database_url();

        let mut config = ManagerConfig::default();
        config.custom_setup = Box::new(connect);

        let connection_manager =
            AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(url, config);
        let pool = Pool::builder()
            .max_size(10)
            .build(connection_manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn get(&self) -> anyhow::Result<ProviderConnection<'_>> {
        self.pool
            .get()
            .await
            .context("Failed to get connection from Postgres pool")
    }
}
