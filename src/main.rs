mod config;
mod routes;
mod services;
mod types;

use routes::access_key_routes::create_asccess_key;
use routes::auth_routes::{login_route, login_with_token_route};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![login_route, login_with_token_route, create_asccess_key],
    )
}
