use crate::api;
use crate::api::twitch::{get_token, get_twitch_id};
use crate::rocket::serde::json::Json;
use crate::services::token_service::save_token;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Code {
    pub code: String,
}

#[post("/token", format = "json", data = "<data>")]
pub async fn post_token(data: Json<Code>) -> Json<bool> {
    let token = get_token(data.code.to_owned()).await;
    let id = get_twitch_id(&token).await;

    let error = save_token(token, id);

    match error {
        Some(val) => panic!("{}", val),
        None => (),
    }

    return Json(true);
}
