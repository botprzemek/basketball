use std::env::var;

pub struct Config {
    // cache_host: String,
    // cache_port: u16,
    // cache_ttl: u64,
    database_host: String,
    database_port: u16,
    database_name: String,

    server_host: String,
    server_port: u16,
}

// pub trait CacheConfig {
//     fn get_cache_url(&self) -> String;
//     fn get_cache_ttl(&self) -> u64;
// }

pub trait DatabaseConfig {
    fn get_database_url(&self) -> String;
}

pub trait ServerConfig {
    fn get_server_url(&self) -> String;
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // let cache_host = var("CACHE_HOST")?;
        // let cache_port = var("CACHE_PORT")?.parse::<u16>()?;
        // let cache_ttl = var("CACHE_TTL")?.parse::<u64>()?;

        let database_host = var("DATABASE_HOST")?;
        let database_port = var("DATABASE_PORT")?.parse::<u16>()?;
        let database_name = var("DATABASE_NAME")?;

        let server_host = var("SERVER_HOST")?;
        let server_port = var("SERVER_PORT")?.parse::<u16>()?;

        Ok(Config {
            // cache_host,
            // cache_port,
            // cache_ttl,
            database_host,
            database_port,
            database_name,
            server_host,
            server_port,
        })
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

impl DatabaseConfig for Config {
    fn get_database_url(&self) -> String {
        format!(
            "postgresql://root@{}:{}/{}?sslmode=disable",
            self.database_host, self.database_port, self.database_name
        )
    }
}

impl ServerConfig for Config {
    fn get_server_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
