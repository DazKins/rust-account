use crate::model::account::{AccountId, Account};

pub mod default;

pub enum AccountFetchError {
    NotFound,
}

pub trait AccountFetcher: Sync + Send {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError>;
}

pub enum AccountCreationError {
    AlreadyExists,
}

pub trait AccountCreator: Sync + Send {
    fn create_account(&self, account_id: AccountId) -> Result<Account, AccountCreationError>;
}
