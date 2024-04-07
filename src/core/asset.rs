use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    Json,
};

use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::json;

use crate::{constants::config::CONFIG_JSON_PATH, utils::json::read_json};

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub hash: String,
}

pub async fn get_file(Path(asset): Path<Asset>) {
    let config = read_json(CONFIG_JSON_PATH);
    let mode = config["server"]["mode"].as_str().unwrap();

    let version = if mode == "cn" {
        config["version"]["android"]["resVersion"].as_str().unwrap()
    } else {
        config["versionGlobal"]["android"]["resVersion"].as_str().unwrap()
    };

    let base_path = format!("./assets/{version}/redirect/");

    if !config["assets"]["downloadLocally"].as_bool().unwrap() {
        let base_path = format!("./assets/{version}/");
        if &asset.name != "hot_update_list.json" {
            if mode == "cn" {
                let name = asset.name;
            } else {
            }
        }
    }
}
