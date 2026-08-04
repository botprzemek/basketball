use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

#[derive(Clone)]
pub struct PasswordService {
    argon2: Argon2<'static>,
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
            let password_hash = PasswordHash::new(&password_hash)?;

            argon2
                .verify_password(password.as_bytes(), &password_hash)
                .map_err(|_| anyhow::anyhow!("Invalid credentials"))
        })
        .await?
    }
}
