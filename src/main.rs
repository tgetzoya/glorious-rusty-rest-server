mod endpoints;
mod models;
mod utils;
mod guards;
mod responders;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![endpoints::index::index])
        .mount("/auth", routes![endpoints::auth::authorize, endpoints::auth::check_auth])
}
