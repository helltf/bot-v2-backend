use std::env;

use reqwest::header::HeaderMap;

use serde::{Deserialize, Serialize};

use crate::reqwest;
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: Vec<String>,
    pub refresh_token: String,
    pub token_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBody {
    client_id: String,
    client_secret: String,
    grant_type: String,
    code: String,
    redirect_uri: String,
}

pub async fn get_twitch_id(token: &TokenData) -> String {
    let client = reqwest::Client::new();

    let res = client
        .get("https://id.twitch.tv/oauth2/validate")
        .headers(get_headers(token.access_token.to_owned()))
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

fn get_token_headers(code: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("client_id", env::var("CLIENT_ID").unwrap().parse().unwrap());
    headers.insert(
        "client_secret",
        env::var("CLIENT_SECRET").unwrap().parse().unwrap(),
    );
    headers.insert("grant_type", "authorization_code".parse().unwrap());
    headers.insert("code", code.parse().unwrap());
    headers.insert(
        "redirect_url",
        "http://localhost:3000/callback".parse().unwrap(),
    );

    return headers;
}

fn get_token_body(code: String) -> TokenBody {
    TokenBody {
        client_id: (env::var("CLIENT_ID").unwrap()),
        client_secret: (env::var("CLIENT_SECRET").unwrap()),
        grant_type: ("authorization_code".to_string()),
        code: (code),
        redirect_uri: ("http://localhost:3000/callback".to_string()),
    }
}

pub async fn get_token(code: String) -> TokenData {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let body_json = serde_urlencoded::to_string(get_token_body(code)).unwrap();

    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .headers(headers)
        .body(body_json)
        .send()
        .await
        .expect("Error while requesting twitch");

    let token_data = res.json::<TokenData>().await.expect("error parsing json");

    return token_data;
}
