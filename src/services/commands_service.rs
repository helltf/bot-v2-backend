use crate::db;
use crate::models::{CommandEntity, WordleWord};
use crate::schema::commands::columns::name;
use crate::schema::{commands, wordle_words};
use diesel::prelude::*;

pub fn get_words() -> Vec<WordleWord> {
    let connection = db::create_connection();

    let words = wordle_words::table
        .load::<WordleWord>(&connection)
        .expect("Failed loading words");

    return words;
}

pub fn get_commands() -> Vec<CommandEntity> {
    let connection = db::create_connection();

    let commands_list = commands::table
        .load::<CommandEntity>(&connection)
        .expect("Failed loading commands");

    return commands_list;
}

pub fn get_command_names() -> Vec<String> {
    let connection = db::create_connection();

    let commands_list = commands::table
        .select(name)
        .order(name)
        .load::<String>(&connection)
        .expect("Failed loading commands");

    return commands_list;
}

pub fn get_command_by_name(command_name: String) -> Option<CommandEntity> {
    let connection = db::create_connection();

    let command_result = commands::table
        .filter(commands::dsl::name.eq_all(command_name))
        .first::<CommandEntity>(&connection)
        .optional()
        .unwrap();

    return command_result;
}
