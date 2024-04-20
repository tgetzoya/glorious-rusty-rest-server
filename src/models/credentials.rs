use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'r> {
    pub username: &'r str,
    pub password: &'r str
}