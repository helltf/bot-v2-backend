#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate reqwest;
extern crate sodiumoxide;

mod api;
mod cors;
mod db;
mod models;
mod routes;
mod schema;
mod services;
mod util;

use cors::CORS;
use routes::commands::{get_command_by_name, get_command_names, get_commands};
use routes::token::post_token;

#[options("/<_..>")]
fn all_options() {}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount(
        "/",
        routes![
            get_command_by_name,
            get_commands,
            post_token,
            all_options,
            get_command_names
        ],
    )
}
