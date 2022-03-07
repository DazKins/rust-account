use crate::model::account::{AccountId, Account};

pub mod default;

pub enum GetAccountError {
    NotFound,
}

pub enum CreateAccountError {
    AlreadyExists
}

pub trait AccountManager: Sync + Send {
    fn get_account(&self, account_id: AccountId) -> Result<Account, GetAccountError>;
    fn create_account(&self, account_id: AccountId) -> Result<Account, CreateAccountError>;
}
