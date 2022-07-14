extern crate diesel;

use diesel::PgConnection;
use rocket::serde::json::Json;
use serde::Serialize;

use self::diesel::prelude::*;
use crate::schema::channel::dsl::*;

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ChannelEntity {
    pub id: i32,
    pub channel_name: String,
    pub allowed: bool,
    pub allowed_live: bool,
    pub joined: bool,
    pub times_connected: i32,
    pub connect_timestamp: i64,
}

impl ChannelEntity {
    pub fn read(connection: &PgConnection) -> Json<Vec<ChannelEntity>> {
        let channels = channel
            .filter(allowed.eq(true))
            .load::<ChannelEntity>(connection)
            .expect("Error loading posts");

        Json(channels)
    }
}
