use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use crate::utils::jwt;

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

impl<'r> ApiKey<'r> {
    pub fn get_key(&self) -> &str {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Forbidden, ApiKeyError::Missing)),
            Some(header) if jwt::validate_token(header) => Outcome::Success(ApiKey(header)),
            Some(_) => Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}