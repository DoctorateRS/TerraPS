use crate::{
    constants::config::CONFIG_JSON_PATH,
    utils::{read_json, update_data},
};
use axum::response::Json;

use serde_json::{json, Value};

// fn random_hash() -> String {
//     let mut pool = "abcdef".chars().collect::<Vec<char>>();
//     let mut rng = rand::thread_rng();
//     pool.shuffle(&mut rng);
//     pool.iter().collect::<String>()
// }

pub async fn prod_android_version() -> Json<Value> {
    let server_config = read_json(CONFIG_JSON_PATH);
    let mode = server_config["server"]["mode"].as_str().unwrap_or("cn");
    match mode {
        "cn" => Json(server_config["version"]["android"].clone()),
        "global" => Json(server_config["versionGlobal"]["android"].clone()),
        _ => Json(server_config["version"]["android"].clone()),
    }
}

pub async fn prod_refresh_config() -> Json<Value> {
    Json(json!({"resVersion": Value::Null}))
}

pub async fn prod_network_config() -> Json<Value> {
    network_config_p1().await;
    let server_config = read_json(CONFIG_JSON_PATH);
    let mode = server_config["server"]["mode"].as_str().unwrap();
    let host = server_config["server"]["host"].as_str().unwrap();
    let port = server_config["server"]["port"].as_u64().unwrap();
    let server = format!("http://{}:{}", host, port);
    let func_ver = server_config["networkConfig"][&mode]["content"]["funcVer"]
        .as_str()
        .unwrap();
    let mut network_config = server_config["networkConfig"][&mode].clone();
    for (index, url) in server_config["networkConfig"][&mode]["content"]["configs"][&func_ver]
        ["network"]
        .as_object()
        .unwrap()
    {
        if url.is_string() {
            if url.as_str().unwrap().contains("{server}") {
                network_config["content"]["configs"][&func_ver]["network"][index] =
                    url.as_str().unwrap().replace("{server}", &server).into();
            }
        } else {
            todo!("Handle non-string network config.")
        }
    }
    Json(network_config)
}

async fn network_config_p1() {
    let mut server_config = read_json(CONFIG_JSON_PATH);

    let version = if server_config["assets"]["autoUpdate"]
        .as_bool()
        .unwrap_or(false)
    {
        update_data("https://ak-conf.hypergryph.com/config/prod/official/Android/version").await
    } else {
        server_config["version"]["android"].clone()
    };

    if version != server_config["version"]["android"] {
        server_config["version"]["android"] = version;
    };
}

pub async fn prod_remote_config() -> Json<Value> {
    Json(read_json(CONFIG_JSON_PATH)["remote"].clone())
}

pub async fn prod_pre_announcement() -> Json<Value> {
    Json(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/preannouncement.meta.json").await)
}

pub async fn prod_announcement() -> Json<Value> {
    Json(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/announcement.meta.json").await)
}
