use crate::{service::account::{AccountFetcher, AccountFetchError}, model::account::{Account, AccountId}};

pub trait AccountManager: Sync + Send {
    fn get_account(&self, account_id: AccountId) -> Result<Account, GetAccountError>;
}

pub struct DefaultAccountManager {
    account_fetcher: Box<dyn AccountFetcher>
}

pub enum GetAccountError {
    NotFound,
}

impl DefaultAccountManager {
    pub fn new(account_fetcher: Box<dyn AccountFetcher>) -> Self {
        DefaultAccountManager{
            account_fetcher
        }
    }
}

impl AccountManager for DefaultAccountManager {
    fn get_account(&self, account_id: AccountId) -> Result<Account, GetAccountError> {
        match self.account_fetcher.fetch_account(account_id) {
            Ok(account) => Ok(account),
            Err(e) => match e {
                AccountFetchError::NotFound => Err(GetAccountError::NotFound)
            }
        }
    }
}

