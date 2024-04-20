use rocket::serde::{json};
use rocket::serde::json::Json;
use crate::models::credentials::Credentials;

#[post("/", format = "json", data = "<credentials>")]
pub async fn authorize(credentials: Json<Credentials<'_>>) -> json::Value {
    json::json!({ "username": credentials.username, "password": credentials.password })
}