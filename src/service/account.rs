use crate::model::account::{AccountId, Account};

pub trait AccountFetcher: Sync + Send {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError>;
}

pub struct DefaultAccountService;

pub enum AccountFetchError {
    NotFound,
}

impl AccountFetcher for DefaultAccountService {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError> {
        return Ok(Account{
            id: account_id,
            name: "Joe Bloggs".to_string()
        })
    }
}