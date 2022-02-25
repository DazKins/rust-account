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

pub struct AccountDto<'a> {
    id: &'a str,
    name: &'a str,
}

// TODO
// impl<'a> AccountDto<'a> {
//     pub fn from_account(account: Account<'a>) -> AccountDto<'a> {
//         AccountDto{
//             id: account.id.to_string().as_str(),
//             name: account.name
//         }
//     }
// }
