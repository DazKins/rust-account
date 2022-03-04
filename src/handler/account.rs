use tide::Body;

use crate::model::account::{Account};

use super::model::account::{AccountIdPathParam, AccountDto};

pub async fn get(req: tide::Request<()>) -> tide::Result {
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
    let account_dto = AccountDto::from_account(Account{
        id: account_id,
        name: "Joe Bloggs".to_string(),
    });
    res.set_body(Body::from_json(&account_dto)?);
    Ok(res)
}
