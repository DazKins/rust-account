#[macro_use] extern crate rocket;
use rocket::http::Status;
use uuid::Uuid;

mod model;

use model::credit::{CreateCreditRequestDto, CreditId, CreateCreditRequest};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[put("/accounts/<account_id_param>")]
fn put_account(account_id_param: &str) -> (Status, &'static str) {
    let account_id = match Uuid::parse_str(account_id_param) {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, "Invalid account ID!")
    };

    return (Status::Created, "")
}

#[post("/accounts/<account_id_param>/credits", data = "<request>")]
fn post_credit(account_id_param: &str, request: &str) -> (Status, &'static str) {
    let account_id = match Uuid::parse_str(account_id_param) {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, "Invalid account ID!")
    };

    let create_credit_request_dto: CreateCreditRequestDto = match serde_json::from_str(request) {
        Ok(val) => val,
        Err(_) => return (Status::BadRequest, "Invalid request body!")
    };

    let credit_create_request = match CreateCreditRequest::from_dto(create_credit_request_dto) {
        Ok(val) => val,
        Err(str) => return (Status::BadRequest, "Invalid request body!")
    };

    return (Status::Created, "")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, put_account, post_credit])
}