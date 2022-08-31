use std::string;

use crate::schema::{commands, twitch_tokens, wordle_words};
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "commands"]
pub struct CommandEntity {
    pub name: String,
    pub counter: i32,
    pub permissions: i32,
    pub description: String,
    pub cooldown: i32,
    pub deleted: bool,
    pub alias: Option<Vec<String>>,
    pub required_params: Vec<String>,
    pub optional_params: Vec<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "wordle_words"]
pub struct WordleWord {
    pub word: String,
    pub is_answer: bool,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "twitch_tokens"]
pub struct TwitchTokenEntity {
    pub id: i32,
    pub token: Vec<u8>,
    pub nonce: Vec<u8>,
    pub user_id: Option<i32>,
    pub refresh_token: String,
}
