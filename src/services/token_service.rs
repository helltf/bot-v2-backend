use crate::schema::twitch_tokens::dsl::*;
use crate::{db, models::TwitchTokenEntity};
use diesel::prelude::*;

pub fn save_token(access_token: String, refresh_token: String, twitch_id: String) -> () {
    let connection = db::create_connection();
    let result = diesel::insert_into(twitch_tokens)
        .values((
            token.eq_all(access_token),
            user_id.eq_all(twitch_id.parse::<i32>().unwrap()),
        ))
        .execute(&connection);
}
