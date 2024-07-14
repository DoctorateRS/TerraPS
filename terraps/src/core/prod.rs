use crate::{
    constants::config::CONFIG_JSON_PATH,
    utils::{game::update_data, json::JSON},
};
use axum::response::Json;
use common_utils::{read_json, write_json, ServerConfig};

use serde_json::json;

pub async fn prod_android_version() -> JSON {
    let server_config = read_json(CONFIG_JSON_PATH);
    let mode = server_config["server"]["mode"].as_str().unwrap_or("cn");
    match mode {
        "cn" => Json(server_config["version"]["android"].clone()),
        "global" => Json(server_config["versionGlobal"]["android"].clone()),
        _ => Json(server_config["version"]["android"].clone()),
    }
}

pub async fn prod_refresh_config() -> JSON {
    Json(json!({"resVersion": null}))
}

pub async fn prod_network_config() -> JSON {
    let mut server_config = read_json(CONFIG_JSON_PATH);

    let version = if server_config["assets"]["autoUpdate"].as_bool().unwrap_or(true) {
        update_data("https://ak-conf.hypergryph.com/config/prod/official/Android/version").await
    } else {
        server_config["version"]["android"].clone()
    };

    if version != server_config["version"]["android"] {
        server_config["version"]["android"] = version;
    };

    write_json(CONFIG_JSON_PATH, &server_config).unwrap_or(());
    let server_cfg = ServerConfig::load().unwrap();
    let mode = server_cfg.mode;
    let host = server_cfg.host;
    let port = server_cfg.port;
    let server = format!("http://{}:{}", host, port);
    let func_ver = server_config["networkConfig"][&mode]["content"]["funcVer"].as_str().unwrap();
    let mut network_config = server_config["networkConfig"][&mode].clone();
    for (index, url) in server_config["networkConfig"][&mode]["content"]["configs"][&func_ver]["network"]
        .as_object()
        .unwrap()
    {
        if url.is_string() && url.as_str().unwrap().contains("{server}") {
            network_config["content"]["configs"][&func_ver]["network"][index] = url.as_str().unwrap().replace("{server}", &server).into();
        }
    }
    Json({
        let content = network_config["content"].to_string();
        json!({
            "sign": "sign",
            "content": content,
        })
    })
}

pub async fn prod_remote_config() -> JSON {
    Json(read_json(CONFIG_JSON_PATH)["remote"].clone())
}

pub async fn prod_pre_announcement() -> JSON {
    Json(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/preannouncement.meta.json").await)
}

pub async fn prod_announcement() -> JSON {
    Json(update_data("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android/announcement.meta.json").await)
}
