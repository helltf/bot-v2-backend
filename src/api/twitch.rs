use crate::reqwest;

pub async fn get_twitch_id(token: String) -> i32 {
    let mut res = reqwest::get("http://httpbin.org/get").await;
    print!("{:?}", res.json());
    return 1;
}
