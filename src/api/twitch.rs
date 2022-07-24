use reqwest::header::HeaderMap;

use serde::Deserialize;

use crate::reqwest;

pub async fn get_twitch_id(token: String) -> String {
    let client = reqwest::Client::new();

    let res = client
        .get("https://id.twitch.tv/oauth2/validate")
        .headers(get_headers(token))
        .send()
        .await
        .expect("Error while requesting twitch");

    let user_data = res
        .json::<TwitchUserData>()
        .await
        .expect("error parsing json");

    return user_data.user_id;
}
#[derive(Debug, Deserialize)]
pub struct TwitchUserData {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    pub expires_in: i32,
}

fn get_headers(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("OAuth {token}").parse().unwrap());
    return headers;
}
