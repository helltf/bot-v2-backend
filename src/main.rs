#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate reqwest;

mod api;
mod cors;
mod db;
mod models;
mod schema;
mod services;
use cors::CORS;
use rocket::{http::Status, response::status, serde::json::Json};
use serde::Deserialize;
use services::commands_service;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AccessToken {
    pub token: String,
}

#[get("/commands")]
async fn get_commands() -> Json<Vec<models::CommandEntity>> {
    let a = api::twitch::get_twitch_id("abc".to_string()).await;
    Json(commands_service::get_commands())
}

#[post("/token", format = "json", data = "<access_token>")]
fn post_token(access_token: Json<AccessToken>) -> Json<bool> {
    return Json(true);
}
#[options("/<_..>")]
fn all_options() {}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![get_commands, post_token, all_options])
}
