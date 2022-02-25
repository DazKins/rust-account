use super::model::account::AccountIdPathParam;

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
    res.set_body(account_id.to_string());
    Ok(res)
}
