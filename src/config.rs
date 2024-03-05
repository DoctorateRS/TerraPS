use serde_json::Value;

use crate::{
    datapath::CONFIG_JSON_PATH,
    utils::{read_json, update_data},
};

pub async fn prod_network_config() {
    let server_config = read_json(CONFIG_JSON_PATH).unwrap();
    let mode = server_config["mode"].as_str().unwrap();
    let server = format!("http://{}:{}", server_config["server"]["host"], server_config["server"]["port"]);
}

pub async fn prod_remote_config() -> Value {
    read_json(CONFIG_JSON_PATH).unwrap()["remote"].clone()
}

pub async fn prod_pre_announcement() -> Value {
    update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/preannouncement.meta.json").await
}

pub async fn prod_announcement() -> Value {
    update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/announcement.meta.json").await
}
