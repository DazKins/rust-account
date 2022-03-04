use std::str::FromStr;

use uuid::Uuid;

use crate::model::account::{AccountId, Account};

pub trait AccountFetcher {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError>;
}

pub struct DefaultAccountService;

pub enum AccountFetchError {
    NotFound,
}

impl AccountFetcher for DefaultAccountService {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError> {
        return Ok(Account{
            id: AccountId::new(Uuid::from_str("b2814873-4066-4aa5-b25f-d95013dacae3").unwrap_or_default()),
            name: "Joe Bloggs".to_string()
        })
    }
}