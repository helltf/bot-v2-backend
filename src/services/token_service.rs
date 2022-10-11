use crate::api::twitch::TokenData;
use crate::db;
use crate::schema::twitch_tokens::dsl::*;
use crate::util::utils::bytes_to_vec;
use diesel::prelude::*;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::Key;
use sodiumoxide::crypto::secretbox::Nonce;
use std::env;

pub fn save_token(token_data: TokenData, twitch_id: String) -> Option<diesel::result::Error> {
    let connection = db::create_connection();
    let box_nonce = secretbox::gen_nonce();

    let boxed_at = encrypt_token(token_data.access_token.to_owned(), &box_nonce);

    let result = diesel::insert_into(twitch_tokens)
        .values((
            token.eq_all(boxed_at),
            user_id.eq_all(twitch_id.parse::<i32>().unwrap()),
            nonce.eq_all(bytes_to_vec(&box_nonce.0)),
            refresh_token.eq_all(token_data.refresh_token),
        ))
        .execute(&connection);

    return match result {
        Ok(_) => None,
        Err(error) => Some(error),
    };
}

fn encrypt_token(enc_token: String, box_nonce: &Nonce) -> Vec<u8> {
    let key_string = env::var("BOX_KEY").expect("Key for encrypting is required");
    let key_bytes = key_string.as_bytes();

    let key = Key::from_slice(key_bytes).unwrap();

    return secretbox::seal(enc_token.as_bytes(), box_nonce, &key);
}
