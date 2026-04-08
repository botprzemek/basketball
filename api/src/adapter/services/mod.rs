mod password;

use password::PasswordService;

use crate::adapter::Registry;
use crate::adapter::net::ResponseCache;
use crate::adapter::repositories::AccountRepository;
use crate::domain::applications::AccountApplication;
use crate::domain::ports::PasswordPort;

pub type AccountService = AccountApplication<AccountRepository, PasswordService>;

pub struct Services {
    account: AccountService,
    cache: ResponseCache,
}

impl Services {
    pub fn new(registry: &Registry) -> Self {
        let account = AccountApplication::new(registry.account.clone(), PasswordService::new());
        let cache = registry.cache.clone();

        Self { account, cache }
    }

    pub fn account(&self) -> &AccountService {
        &self.account
    }

    pub fn cache(&self) -> &ResponseCache {
        &self.cache
    }
}
