use serde::Serialize;
use uuid::Uuid;

use crate::model::account::{AccountId, Account};

pub struct AccountIdPathParam<'a>(&'a str);

impl<'a> AccountIdPathParam<'a> {
    pub fn new(s: &'a str) -> Self {
        AccountIdPathParam(s)
    }

    pub fn to_account_id(&self) -> Option<AccountId> {
        match Uuid::parse_str(self.0) {
            Ok(uuid) => Some(AccountId::new(uuid)),
            Err(_) => None,
        }
    }
}

#[derive(Serialize)]
pub struct AccountDto {
    id: String,
    name: String,
}

impl AccountDto {
    pub fn from_account(account: Account) -> AccountDto {
        AccountDto{
            id: account.id.to_string(),
            name: account.name
        }
    }
}
