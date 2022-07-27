use crate::schema::twitch_tokens::dsl::*;
use crate::{db, models::TwitchTokenEntity};
use diesel::prelude::*;
use dotenv;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::Key;
use sodiumoxide::crypto::secretbox::Nonce;
use std::env;

pub fn save_token(access_token: String, save_refresh_token: String, twitch_id: String) -> () {
    let connection = db::create_connection();
    let box_nonce = secretbox::gen_nonce();

    let boxed_at = encrypt_token(access_token, &box_nonce);

    let result = diesel::insert_into(twitch_tokens)
        .values((
            token.eq_all(boxed_at),
            user_id.eq_all(twitch_id.parse::<i32>().unwrap()),
            nonce.eq_all(crate::bytes_to_vec(&box_nonce.0)),
        ))
        .execute(&connection);
}

fn encrypt_token(enc_token: String, box_nonce: &Nonce) -> Vec<u8> {
    let key_string = env::var("BOX_KEY").expect("Key for encrypting is required");
    let key_bytes = key_string.as_bytes();

    let key = Key::from_slice(key_bytes).unwrap();

    return secretbox::seal(enc_token.as_bytes(), box_nonce, &key);
}
