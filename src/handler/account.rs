use std::sync::Arc;

use tide::Body;

use crate::{app::account::{AccountManager, GetAccountError}};

use super::model::account::{AccountIdPathParam, AccountDto};

pub async fn get(req: tide::Request<()>, account_manager: Arc<dyn AccountManager>) -> tide::Result {
    let account_id = match req.param("accountId") {
        Ok(s) => {
            let path_param = AccountIdPathParam::new(s);
            match path_param.to_account_id() {
                Some(account_id) => account_id,
                None => return Ok(tide::Response::new(tide::StatusCode::BadRequest))
            }
        },
        Err(_) => return Ok(tide::Response::new(tide::StatusCode::BadRequest))
    };

    let mut res = tide::Response::new(tide::StatusCode::Ok);

    let account = match account_manager.get_account(account_id) {
        Ok(a) => a,
        Err(e) => match e {
            GetAccountError::NotFound => return Ok(tide::Response::new(tide::StatusCode::NotFound))
        }
    };
    
    let account_dto = AccountDto::from_account(account);
    res.set_body(Body::from_json(&account_dto)?);
    Ok(res)
}
