use crate::{
    datapath::CONFIG_JSON_PATH,
    utils::{read_json, update_data},
};
use axum::response::Response;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

fn random_hash() -> String {
    let mut pool = "abcdef".chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    pool.shuffle(&mut rng);
    pool.iter().collect::<String>()
}

pub async fn prod_refresh_config() -> Response<Value> {
    Response::builder().status(200).body(json!({"resVersion": Value::Null})).unwrap()
}

pub async fn prod_network_config() {
    let mut server_config = read_json(CONFIG_JSON_PATH).unwrap();
    let mode = server_config["server"]["mode"].to_string();
    let server = format!("http://{}:{}", server_config["server"]["host"], server_config["server"]["port"]).as_str();
    let network_config = server_config["networkConfig"][&mode].clone();
    let func_ver = network_config["content"]["funcVer"].to_string();

    let version = if server_config["assets"]["autoUpdate"].as_bool().unwrap_or(false) {
        if mode == "cn" {
            update_data("https://ak-conf.hypergryph.com/config/prod/official/Android/version").await
        } else {
            todo!()
        }
    } else {
        server_config["version"]["android"].clone()
    };

    if version != server_config["version"]["android"] {
        server_config["version"]["android"] = version;
    };

    for (index, value) in network_config["content"]["configs"][&func_ver]["network"].as_object().unwrap() {
        let url = network_config["content"]["configs"][&func_ver]["network"].clone();
        if url.is_string() && url.as_str().unwrap().contains("{server}") {}
    }
}

pub async fn prod_remote_config() -> Response<Value> {
    Response::builder().body(read_json(CONFIG_JSON_PATH).unwrap()["remote"].clone()).unwrap()
}

pub async fn prod_pre_announcement() -> Response<Value> {
    Response::builder()
        .body(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/preannouncement.meta.json").await)
        .unwrap()
}

pub async fn prod_announcement() -> Response<Value> {
    Response::builder()
        .body(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/announcement.meta.json").await)
        .unwrap()
}
