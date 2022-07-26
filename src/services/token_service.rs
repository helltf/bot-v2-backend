use crate::schema::twitch_tokens::dsl::*;
use crate::{db, models::TwitchTokenEntity};
use diesel::prelude::*;
use sodiumoxide::crypto::secretbox;

pub fn save_token(access_token: String, save_refresh_token: String, twitch_id: String) -> () {
    let connection = db::create_connection();
    let box_nonce = secretbox::gen_nonce();
    let boxed_at = secretbox::seal(access_token.as_bytes(), &box_nonce, &secretbox::gen_key());
    let result = diesel::insert_into(twitch_tokens)
        .values((
            token.eq_all(boxed_at),
            user_id.eq_all(twitch_id.parse::<i32>().unwrap()),
            nonce.eq_all(crate::bytes_to_vec(&box_nonce.0)),
        ))
        .execute(&connection);
}
