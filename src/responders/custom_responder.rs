use rocket::http::{Header, Status};
use rocket::response::Responder;
use rocket::serde::json::Value;

use crate::utils::jwt;

#[derive(Responder)]
#[response(content_type = "json")]
pub struct CustomResponder {
    inner: (Status, Value),
    authorization: Header<'static>,
}

impl CustomResponder {
    pub fn new(status: Status, response: Value, key: &str) -> CustomResponder {
        CustomResponder {
            inner: (status, response),
            authorization: Header::new("Authorization", jwt::update_token(key)),
        }
    }
}