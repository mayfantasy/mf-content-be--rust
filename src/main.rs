mod config;
mod routes;
mod services;

use routes::accessKeyRoutes::create_asccess_key;
use routes::loginRoutes::{login_route, login_with_token_route};

#[macro_use]
extern crate rocket;

use faunadb::prelude::*;
use futures::{future::lazy, Future};
use tokio;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![login_route, login_with_token_route, create_asccess_key],
    )
}
