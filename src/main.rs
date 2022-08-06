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
use rocket::serde::json::Json;
use routes::commands::get_commands;
use routes::token::post_token;
use serde::Deserialize;
use services::{commands_service, token_service::save_token};

#[options("/<_..>")]
fn all_options() {}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![get_commands, post_token, all_options])
}
