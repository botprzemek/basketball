use chrono::Utc;
use uuid::Uuid;

use crate::domain::entities::Account;
use crate::domain::ports::AccountPort;
use crate::domain::ports::PasswordPort;

pub struct AccountApplication<A: AccountPort, P: PasswordPort> {
    account_service: A,
    password_service: P,
}

impl<A: AccountPort, P: PasswordPort> AccountApplication<A, P> {
    pub fn new(account_service: A, password_service: P) -> Self {
        Self {
            account_service,
            password_service,
        }
    }

    pub async fn find_all(&self, _page: Option<usize>, per_page: Option<usize>) -> anyhow::Result<Vec<Account>> {
        self.account_service.select_all(per_page).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        self.account_service.select_by_id(id).await
    }

    pub async fn create(
        &self,
        password: String,
        first_name: String,
        last_name: String,
    ) -> anyhow::Result<Account> {
        let password_hash = self.password_service.generate(password).await?;
        let account = Account::new(password_hash, first_name, last_name);

        self.account_service.insert(account).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> anyhow::Result<Option<Account>> {
        let mut account = match self.account_service.select_by_id(id).await? {
            Some(account) => account,
            None => return Ok(None),
        };

        let mut has_changed = false;

        if let Some(password) = password {
            account.password_hash = self.password_service.generate(password).await?;
            has_changed = true;
        }

        if let Some(first_name) = first_name {
            account.first_name = first_name;
            has_changed = true;
        }

        if let Some(last_name) = last_name {
            account.last_name = last_name;
            has_changed = true;
        }

        if has_changed {
            account.updated_at = Some(Utc::now());
            account = self.account_service.update(account).await?;
        }

        Ok(Some(account))
    }

    pub async fn update_password(&self, id: Uuid, password: String) -> anyhow::Result<()> {
        let Some(mut account) = self.find_by_id(id).await? else {
            return Ok(());
        };

        account.password_hash = self.password_service.generate(password).await?;
        account.updated_at = Some(Utc::now());

        let _ = self.account_service.update(account).await?;

        Ok(())
    }

    pub async fn verify_password(&self, id: Uuid, password: String) -> anyhow::Result<bool> {
        let Some(account) = self.find_by_id(id).await? else {
            return Ok(false);
        };

        match self
            .password_service
            .verify(password, account.password_hash)
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.account_service.delete(id).await
    }
}
