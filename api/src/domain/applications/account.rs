use crate::domain::entities::Account;
use crate::domain::ports::AccountPort;

pub struct AccountApplication<A: AccountPort> {
    account_service: A,
}

pub struct CreateAccount {
    pub email: String,
    pub password_hash: String,
}

impl<A: AccountPort> AccountApplication<A> {
    pub fn new(account_service: A) -> Self {
        Self { account_service }
    }

    pub async fn find_by_email(&self, email: String) -> anyhow::Result<Option<Account>> {
        self.account_service.select_by_email(email).await
    }

    pub async fn create(&self, command: CreateAccount) -> anyhow::Result<Account> {
        let account = Account::new(command.email, command.password_hash);

        self.account_service.insert(account).await
    }
}
