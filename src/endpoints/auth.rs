use rocket::http::{Status};
use rocket::serde::json;
use rocket::serde::json::Json;

use crate::guards::requests::ApiKey;
use crate::models::credentials::Credentials;
use crate::responders::custom_responder::CustomResponder;
use crate::utils::jwt;

#[post("/", format = "json", data = "<credentials>")]
pub async fn authorize(credentials: Json<Credentials<'_>>) -> json::Value {
    json::json!({ "token": jwt::generate_token(credentials.username) })
}

#[get("/check")]
pub async fn check_auth<'r>(key: ApiKey<'_>) -> CustomResponder {
    let bob = json::json!({ "status": "acceptable" });

    CustomResponder::new(Status::Accepted, bob, key.get_key())
}