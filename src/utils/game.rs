use super::{battle_data::BattleDataDecoder, json::read_json};
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

pub fn decrypt_battle_data(data: &str, login_time: Option<u64>) -> Value {
    let decryptor = match login_time {
        Some(time) => BattleDataDecoder::new_with_login_time(time as u32),
        None => BattleDataDecoder::new(),
    };
    decryptor.decrypt_battle_data(data.to_string()).unwrap()
}
