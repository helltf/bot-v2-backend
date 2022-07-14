#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod commands_service;
mod db;
mod models;
mod schema;

use rocket::serde::json::Json;

#[get("/commands")]
fn get_commands() -> Json<Vec<models::WordleWord>> {
    Json(commands_service::get_commands())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_commands])
}
