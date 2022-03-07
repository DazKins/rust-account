use std::sync::Arc;

use crate::{service::account::{AccountFetcher, AccountFetchError, AccountCreator, AccountCreationError}, model::account::{AccountId, Account}};

use super::{AccountManager, GetAccountError, CreateAccountError};

pub struct DefaultAccountManager {
    account_fetcher: Arc<dyn AccountFetcher>,
    account_creator: Arc<dyn AccountCreator>
}

impl DefaultAccountManager {
    pub fn new(account_fetcher: Arc<dyn AccountFetcher>, account_creator: Arc<dyn AccountCreator>) -> Self {
        DefaultAccountManager{
            account_fetcher,
            account_creator
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

    fn create_account(&self, account_id: AccountId) -> Result<Account, super::CreateAccountError> {
        match self.account_creator.create_account(account_id) {
            Ok(account) => Ok(account),
            Err(e) => match e {
                AccountCreationError::AlreadyExists => Err(CreateAccountError::AlreadyExists)
            }
        }
    }
}
