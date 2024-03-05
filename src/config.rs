use crate::{
    datapath::CONFIG_JSON_PATH,
    utils::{read_json, update_data},
};
use axum::Json;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

fn random_hash() -> String {
    let mut pool = "abcdef".chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    pool.shuffle(&mut rng);
    pool.iter().collect::<String>()
}

pub async fn prod_refresh_config() {
    // data = {"resVersion": None}
    // return data, 200 <- todo: figure out what the fuck is this
}

pub async fn prod_network_config() {
    let server_config = read_json(CONFIG_JSON_PATH).unwrap();
    let server = format!("http://{}:{}", server_config["server"]["host"], server_config["server"]["port"]).as_str();
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
