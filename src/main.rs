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
fn get_commands() -> Json<Vec<models::CommandEntity>> {
    Json(commands_service::get_commands())
}

#[post("/token", format = "json", data = "<access_token>")]
async fn post_token(access_token: Json<AccessToken>) -> Json<bool> {
    let a = api::twitch::get_twitch_id(access_token.token.to_owned()).await;
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
