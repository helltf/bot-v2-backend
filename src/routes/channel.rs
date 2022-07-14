use crate::models::channel::ChannelEntity;
use diesel::PgConnection;
use rocket::{get, response::status, serde::json::Json};

#[get("/channels")]
pub fn get_all_channels(connection: &PgConnection) -> Json<Vec<ChannelEntity>> {
    let data = ChannelEntity::read(connection);
    return data;
}
