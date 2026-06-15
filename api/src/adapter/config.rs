use std::env::var;

pub struct Config {
    server_host: String,
    server_port: u16,
    server_base_url: String,

    token_issuer: String,
    token_secret: String,

    database_host: String,
    database_port: u16,
    database_name: String,
    database_username: String,
    database_password: String,
    //
    // cache_host: String,
    // cache_port: u16,
    // cache_ttl: u64,
}

pub trait ServerConfig {
    fn server_url(&self) -> String;
}

pub trait TokenConfig {
    fn token_issuer(&self) -> String;
    fn token_secret(&self) -> String;
}

pub trait DatabaseConfig {
    fn database_url(&self) -> String;
}

// pub trait CacheConfig {
//     fn cache_url(&self) -> String;
//     fn cache_ttl(&self) -> u64;
// }

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let server_host = var("API_SERVER_HOST")?;
        let server_port = var("API_SERVER_PORT")?.parse::<u16>()?;
        let server_base_url = var("API_SERVER_BASE_URL")?;

        let token_issuer = var("API_TOKEN_ISSUER")?;
        let token_secret = var("API_TOKEN_SECRET")?;

        let database_host = var("API_DATABASE_HOST")?;
        let database_port = var("API_DATABASE_PORT")?.parse::<u16>()?;
        let database_name = var("API_DATABASE_NAME")?;
        let database_username = var("API_DATABASE_USERNAME")?;
        let database_password = var("API_DATABASE_PASSWORD")?;

        // let cache_host = var("CACHE_HOST")?;
        // let cache_port = var("CACHE_PORT")?.parse::<u16>()?;
        // let cache_ttl = var("CACHE_TTL")?.parse::<u64>()?;

        Ok(Config {
            server_host,
            server_port,
            server_base_url,

            token_issuer,
            token_secret,

            database_host,
            database_port,
            database_name,
            database_username,
            database_password,
            //
            // cache_host,
            // cache_port,
            // cache_ttl,
        })
    }
}

impl ServerConfig for Config {
    fn server_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

impl DatabaseConfig for Config {
    fn database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?application_name=auth&sslmode=disable",
            self.database_username,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name,
        )
        // format!(
        //     "postgresql://{}:{}@{}:{}/{}?sslmode=verify-full&sslcert=/workspace/auth/cockroach/certs/client.{}.crt&sslkey=/workspace/auth/cockroach/certs/client.{}.key&sslrootcert=/workspace/auth/cockroach/certs/ca.crt",
        //     self.database_username,
        //     self.database_password,
        //     self.database_host,
        //     self.database_port,
        //     self.database_name,
        //     self.database_username,
        //     self.database_username
        // )
    }
}

impl TokenConfig for Config {
    fn token_issuer(&self) -> String {
        self.token_issuer.clone()
    }
    fn token_secret(&self) -> String {
        self.token_secret.clone()
    }
}

// impl CacheConfig for Config {
//     fn cache_url(&self) -> String {
//         format!("redis://{}:{}/", self.cache_host, self.cache_port)
//     }

//     fn cache_ttl(&self) -> u64 {
//         self.cache_ttl
//     }
// }
