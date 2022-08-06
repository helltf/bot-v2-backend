use crate::api;
use crate::rocket::serde::json::Json;
use crate::services::token_service::save_token;
use serde::Deserialize;
#[derive(Debug, PartialEq, Eq, Deserialize)]

pub struct TokenData {
    pub token: String,
    pub refresh_token: String,
}

#[post("/token", format = "json", data = "<data>")]
pub async fn post_token(data: Json<TokenData>) -> Json<bool> {
    let id = api::twitch::get_twitch_id(data.token.to_owned()).await;
    save_token(data.token.to_owned(), data.refresh_token.to_owned(), id);

    return Json(true);
}
