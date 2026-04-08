use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;

use crate::domain::ports::PasswordPort;

#[derive(Clone)]
pub struct PasswordService {
    argon2: Argon2<'static>,
}

#[async_trait]
impl PasswordPort for PasswordService {
    fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    async fn generate(&self, password: String) -> anyhow::Result<String> {
        let argon2 = self.argon2.clone();

        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);

            Ok(argon2
                .hash_password(password.as_bytes(), &salt)?
                .to_string())
        })
        .await?
    }

    async fn verify(&self, password: String, password_hash: String) -> anyhow::Result<()> {
        let argon2 = self.argon2.clone();

        tokio::task::spawn_blocking(move || {
            let password_hash = PasswordHash::new(&password_hash)?;

            Ok(argon2.verify_password(password.as_bytes(), &password_hash)?)
        })
        .await?
    }
}
