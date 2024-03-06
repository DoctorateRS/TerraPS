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
    return Response::builder().status(200).body(json!({"resVersion": Value::Null})).unwrap();
}

pub async fn prod_network_config() {
    let server_config = read_json(CONFIG_JSON_PATH).unwrap();
    let _server = format!("http://{}:{}", server_config["server"]["host"], server_config["server"]["port"]).as_str();
}

pub async fn prod_remote_config() -> Response<Value> {
    Response::builder().body(read_json(CONFIG_JSON_PATH).unwrap()["remote"].clone()).unwrap()
}

pub async fn prod_pre_announcement() -> Response<Value> {
    Response::builder().body(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/preannouncement.meta.json").await).unwrap()
}

pub async fn prod_announcement() -> Response<Value> {
    Response::builder().body(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/announcement.meta.json").await).unwrap()
}
