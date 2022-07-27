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
mod schema;
mod services;
use cors::CORS;
use rocket::serde::json::Json;
use serde::Deserialize;
use services::{commands_service, token_service::save_token};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TokenData {
    pub token: String,
    pub refresh_token: String,
}

pub fn bytes_to_vec(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());

    out.extend_from_slice(b);
    out
}

#[get("/commands")]
fn get_commands() -> Json<Vec<models::CommandEntity>> {
    Json(commands_service::get_commands())
}

#[post("/token", format = "json", data = "<data>")]
async fn post_token(data: Json<TokenData>) -> Json<bool> {
    let id = api::twitch::get_twitch_id(data.token.to_owned()).await;
    save_token(data.token.to_owned(), data.refresh_token.to_owned(), id);

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
