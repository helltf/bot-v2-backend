use diesel::RunQueryDsl;

use crate::{db, models::TwitchTokenEntity, schema::twitch_tokens};

pub fn save_token(token: String) -> bool {
    let connection = db::create_connection();

    // let words = diesel::insert_into(twitch_tokens).values(records)
    //     .execute(&connection)
    //     .expect("Error saving token");

    return true;
}
