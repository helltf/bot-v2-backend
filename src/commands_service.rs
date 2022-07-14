use crate::db;
use crate::models::{CommandEntity, WordleWord};
use crate::schema::channel::allowed;
use crate::schema::wordle_words;
use diesel::prelude::*;

pub fn get_commands() -> Vec<WordleWord> {
    let connection = db::create_connection();

    let words = wordle_words::table
        .load::<WordleWord>(&connection)
        .expect("Failed loading words");

    return words;
}
