use common_utils::read_json;
use reqwest::get;
use serde_json::Value;

#[deprecated]
pub async fn update_data(url: &str) -> Value {
    let local_path = url
        .replace("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata", "./data")
        .replace("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android", "./data/announce");

    if url.contains("Android/version") {
        match get(url).await.unwrap().json::<Value>().await {
            Ok(value) => value,
            Err(_) => panic!("Unable to process request."),
        }
    } else {
        read_json(local_path)
    }
}
