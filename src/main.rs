extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
use bot_v2_backend::routes::channel::get_all_channels;

extern crate diesel;

mod schema;

use crate::schema::todo;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use rocket::{self, get, post, put, routes};
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to read .env file");
    db::connect();
    rocket::build().mount("/", routes![get_all_channels]);
}
