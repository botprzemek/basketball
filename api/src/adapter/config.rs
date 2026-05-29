use std::env::var;

pub struct Config {
    server_host: String,
    server_port: u16,

    database_host: String,
    database_port: u16,
    database_name: String,
    
    // cache_host: String,
    // cache_port: u16,
    // cache_ttl: u64,
}

pub trait ServerConfig {
    fn get_server_url(&self) -> String;
}

pub trait DatabaseConfig {
    fn get_database_url(&self) -> String;
}

// pub trait CacheConfig {
//     fn get_cache_url(&self) -> String;
//     fn get_cache_ttl(&self) -> u64;
// }

impl Config {
    pub fn load() -> anyhow::Result<Self> {

        let server_host = var("API_SERVER_HOST")?;
        let server_port = var("API_SERVER_PORT")?.parse::<u16>()?;

        let database_host = var("API_DATABASE_HOST")?;
        let database_port = var("API_DATABASE_PORT")?.parse::<u16>()?;
        let database_name = var("API_DATABASE_NAME")?;

        // let cache_host = var("CACHE_HOST")?;
        // let cache_port = var("CACHE_PORT")?.parse::<u16>()?;
        // let cache_ttl = var("CACHE_TTL")?.parse::<u64>()?;

        Ok(Config {
            server_host,
            server_port,

            database_host,
            database_port,
            database_name,

            // cache_host,
            // cache_port,
            // cache_ttl,
        })
    }
}

impl ServerConfig for Config {
    fn get_server_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

impl DatabaseConfig for Config {
    fn get_database_url(&self) -> String {
        format!(
            "postgresql://root@{}:{}/{}?sslmode=disable",
            self.database_host, self.database_port, self.database_name
        )
    }
}

// impl CacheConfig for Config {
//     fn get_cache_url(&self) -> String {
//         format!("redis://{}:{}/", self.cache_host, self.cache_port)
//     }

//     fn get_cache_ttl(&self) -> u64 {
//         self.cache_ttl
//     }
// }
