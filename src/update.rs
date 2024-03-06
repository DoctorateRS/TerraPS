use crate::{
    datapath::CONFIG_JSON_PATH,
    utils::{read_json, write_json},
};
use reqwest::get;
use serde_json::Value;

pub async fn update_config() {
    let mut config = read_json(CONFIG_JSON_PATH).unwrap();

    let old_res_ver = config["version"]["android"]["resVersion"].clone();
    let old_client_ver = config["version"]["android"]["clientVersion"].clone();

    let old_func_ver = config["networkConfig"]["cn"]["content"]["funcVer"].clone();

    let official_response = match get("https://ak-conf.hypergryph.com/config/prod/official/Android/version").await {
        Ok(res) => res.json::<Value>().await.unwrap(),
        Err(_) => panic!("Unable to parse request."),
    };

    let new_res_ver = official_response["resVersion"].clone();
    let new_client_ver = official_response["clientVersion"].clone();

    if new_res_ver != old_res_ver {
        config["version"]["android"]["resVersion"] = new_res_ver;
    }
    if new_client_ver != old_client_ver {
        config["version"]["android"]["clientVersion"] = new_client_ver;
    }
}
