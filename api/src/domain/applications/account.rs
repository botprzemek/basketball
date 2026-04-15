use chrono::Utc;
use uuid::Uuid;

use crate::domain::{entities::Account, ports::AccountPort};

pub struct AccountApplication<A: AccountPort> {
    account_service: A,
}

pub struct CreateAccount {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct UpdateAccount {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl<A: AccountPort> AccountApplication<A> {
    pub fn new(account_service: A) -> Self {
        Self { account_service }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Account>> {
        self.account_service.select().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        self.account_service.select_by_self(id).await
    }

    pub async fn find_by_email(&self, email: String) -> anyhow::Result<Option<Account>> {
        self.account_service.select_by_email(email).await
    }

    pub async fn create(&self, command: CreateAccount) -> anyhow::Result<Account> {
        let account = Account::new(
            command.email,
            command.password_hash,
            command.first_name,
            command.last_name,
        );

        self.account_service.insert(account).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        command: UpdateAccount,
    ) -> anyhow::Result<Option<Account>> {
        let mut account = match self.account_service.select_by_self(id).await? {
            Some(account) => account,
            None => return Ok(None),
        };
        let mut has_changed = false;

        if let Some(email) = command.email {
            account.email = email;
            has_changed = true;
        }

        if let Some(first_name) = command.first_name {
            account.first_name = first_name;
            has_changed = true;
        }

        if let Some(last_name) = command.last_name {
            account.last_name = last_name;
            has_changed = true;
        }

        if has_changed {
            account.updated_at = Some(Utc::now());
            account = self.account_service.update(account).await?;
        }

        Ok(Some(account))
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.account_service.delete(id).await
    }
}
