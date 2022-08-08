use crate::models::CommandEntity;
use crate::services::commands_service;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

#[get("/commands")]
pub fn get_commands() -> Json<Vec<CommandEntity>> {
    Json(commands_service::get_commands())
}

#[get("/commands/name")]
pub fn get_command_names() -> Json<Vec<String>> {
    Json(commands_service::get_command_names())
}

#[get("/command/name/<name>")]
pub fn get_command_by_name(name: String) -> Option<Json<CommandEntity>> {
    let command = commands_service::get_command_by_name(name);
    match command {
        Some(c) => return Some(Json(c)),
        None => return None,
    }
}
