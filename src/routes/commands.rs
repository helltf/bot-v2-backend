use crate::models::CommandEntity;
use crate::services::commands_service;
use rocket::serde::json::Json;

#[get("/commands")]
pub fn get_commands() -> Json<Vec<CommandEntity>> {
    Json(commands_service::get_commands())
}
