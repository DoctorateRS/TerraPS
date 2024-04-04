use super::json::read_json;
use crate::utils::crypto::{hex::from_hex, md5::md5_digest};
use reqwest::get;
use serde_json::Value;

pub async fn update_data(url: &str) -> Value {
    let local_path = url
        .replace(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata",
            "./data",
        )
        .replace(
            "https://ak-conf.hypergryph.com/config/prod/announce_meta/Android",
            "./data/announce",
        );

    if url.contains("Android/version") {
        match get(url).await.unwrap().json::<Value>().await {
            Ok(value) => value,
            Err(_) => panic!("Unable to process request."),
        }
    } else {
        read_json(&local_path)
    }
}

pub fn decrypt_battle_data(data: &str, login_time: Option<u64>) {
    const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

    let login_time = login_time.unwrap_or(1672502400);
    let ptr = &data.len() - 32;
    let iv = &data[ptr..];
    let data = &data[..ptr];

    let src = format!("{LOG_TOKEN_KEY}{login_time}");
    let key = md5_digest(src.as_bytes());

    let battle_data = match from_hex(data) {
        Ok(data) => match String::from_utf8(data) {
            Ok(data) => data,
            Err(e) => panic!("Error parsing UTF-8: {e}"),
        },
        Err(e) => panic!("Error parsing Integer:{}", e),
    };
}
