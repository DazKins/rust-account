use uuid::Uuid;
use crate::model::account::{AccountId, Account};
use super::{AccountFetcher, AccountFetchError, AccountCreator, AccountCreationError};

// pub struct DefaultAccountService {
//     postgres_client: postgres::Client,
// }
pub struct DefaultAccountService;

impl AccountFetcher for DefaultAccountService {
    fn fetch_account(&self, account_id: AccountId) -> Result<Account, AccountFetchError> {
        //"261fd4f9-8a88-45f9-a47d-486cb03e2046"
        static TEST_ACCOUNT_ID: AccountId = AccountId::new(Uuid::from_u128(50675944683963498747971562123512651846));

        if account_id == TEST_ACCOUNT_ID {
            return Err(AccountFetchError::NotFound)
        }

        return Ok(Account {
            id: account_id,
            name: "Joe Bloggs".to_string()
        })
    }
}

impl AccountCreator for DefaultAccountService {
    fn create_account(&self, account_id: AccountId) -> Result<Account, AccountCreationError> {
        //"261fd4f9-8a88-45f9-a47d-486cb03e2046"
        static TEST_ACCOUNT_ID: AccountId = AccountId::new(Uuid::from_u128(50675944683963498747971562123512651846));

        if account_id == TEST_ACCOUNT_ID {
            return Err(AccountCreationError::AlreadyExists)
        }

        return Ok(Account{
            id: account_id,
            name: "Joe Bloggs".to_string()
        })
    }
}
