mod endpoints;
mod models;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![endpoints::index::index])
        .mount("/auth", routes![endpoints::auth::authorize])
}
