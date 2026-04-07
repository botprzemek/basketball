use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use std::sync::Arc;

use crate::adapter::Registry;
use crate::adapter::net::ResponseCache;
use crate::adapter::repositories::AccountRepository;
use crate::domain::applications::AccountApplication;

pub type AccountService = AccountApplication<AccountRepository>;

pub struct Services {
    account: Arc<AccountService>,
    password: Arc<PasswordService>,
    cache: Arc<ResponseCache>,
}

pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl Services {
    pub fn new(registry: Arc<Registry>) -> Self {
        let password = Arc::new(PasswordService::new());
        let account = Arc::new(AccountApplication::new(
            registry.account(),
            password.clone(),
        ));
        let cache = registry.cache();

        Self {
            account,
            password,
            cache,
        }
    }

    pub fn account(&self) -> Arc<AccountService> {
        self.account.clone()
    }

    pub fn password(&self) -> Arc<PasswordService> {
        self.password.clone()
    }

    pub fn cache(&self) -> Arc<ResponseCache> {
        self.cache.clone()
    }
}

impl PasswordService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub async fn generate(&self, password: String) -> anyhow::Result<String> {
        let argon2 = self.argon2.clone();

        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);

            Ok(argon2
                .hash_password(password.as_bytes(), &salt)?
                .to_string())
        })
        .await?
    }

    pub async fn verify(&self, password: String, password_hash: String) -> anyhow::Result<()> {
        let argon2 = self.argon2.clone();

        tokio::task::spawn_blocking(move || {
            let hash = PasswordHash::new(&password_hash)?;

            Ok(argon2.verify_password(password.as_bytes(), &hash)?)
        })
        .await?
    }
}
