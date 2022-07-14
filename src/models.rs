use std::string;

use crate::schema::{command, wordle_words};
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "command"]
pub struct CommandEntity {
    pub name: String,
    pub counter: i32,
    pub permissions: i32,
    pub description: String,
    pub requiredParams: Vec<String>,
    pub optionalParams: Vec<String>,
    pub cooldown: i32,
    pub deleted: bool,
    pub alias: Option<Vec<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "wordle_words"]
pub struct WordleWord {
    pub word: String,
    pub is_answer: bool,
}
