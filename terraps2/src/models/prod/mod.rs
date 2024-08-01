use std::collections::HashMap;

use common_utils::read_json;
use serde::{Deserialize, Serialize};
use serde_json::from_value;

use crate::{cnst::config::CONFIG_PATH, SERVER_CONFIG};

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidVersion {
    #[serde(rename = "resVersion")]
    res_ver: String,
    #[serde(rename = "clientVersion")]
    clnt_ver: String,
}

impl ProdAndroidVersion {
    pub fn load() -> Self {
        if SERVER_CONFIG.mode == "cn" {
            from_value(read_json(CONFIG_PATH)["version"]["android"].clone()).unwrap()
        } else {
            from_value(read_json(CONFIG_PATH)["versionGlobal"]["android"].clone()).unwrap()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidRefresh {
    #[serde(rename = "resVersion")]
    res_ver: HashMap<(), ()>,
}

impl ProdAndroidRefresh {
    pub fn default() -> Self {
        Self { res_ver: HashMap::default() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidNetwork {
    sign: String,
    content: String,
}

impl ProdAndroidNetwork {
    pub fn load() -> Self {
        let mode = &SERVER_CONFIG.mode;
        let host = &SERVER_CONFIG.host;
        let port = SERVER_CONFIG.port;

        let cfg = read_json(CONFIG_PATH);

        let server = format!("http://{}:{}", host, port);
        let func_ver = cfg["networkConfig"][&mode]["content"]["funcVer"].as_str().unwrap_or("");

        let mut network_config = cfg["networkConfig"][&mode].clone();
        for (index, url) in cfg["networkConfig"][&mode]["content"]["configs"][&func_ver]["network"].as_object().unwrap() {
            if url.is_string() && url.as_str().unwrap().contains("{server}") {
                network_config["content"]["configs"][&func_ver]["network"][index] = url.as_str().unwrap().replace("{server}", &server).into();
            }
        }

        let content = network_config["content"].to_string();

        Self { sign: String::from("sign"), content }
    }
}

pub type ProdAndroidRemote = HashMap<(), ()>;

#[derive(Serialize, Deserialize)]
struct Announce {
    #[serde(rename = "announceId")]
    id: String,
    day: i64,
    group: String,
    #[serde(rename = "isWebUrl")]
    is_url: bool,
    month: i64,
    title: String,
    #[serde(rename = "webUrl")]
    url: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Extra {
    enable: bool,
    name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AnnouncementMeta {
    #[serde(rename = "announceList")]
    list: Vec<Announce>,
    extra: Extra,
    #[serde(rename = "focusAnnounceId")]
    focus_id: (),
}

impl AnnouncementMeta {
    pub fn load() -> Self {
        let raw = read_json("./data/announce/announcement.meta.json");
        from_value(raw).unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct PreannouncementMeta {
    actived: bool,
    #[serde(rename = "preAnnounceId")]
    id: String,
    #[serde(rename = "preAnnounceType")]
    t: i64,
    #[serde(rename = "preAnnounceUrl")]
    url: String,
}

impl PreannouncementMeta {
    pub fn load() -> Self {
        let raw = read_json("./data/announce/preannouncement.meta.json");
        from_value(raw).unwrap_or_default()
    }
}

impl Default for PreannouncementMeta {
    fn default() -> Self {
        Self {
            actived: true,
            id: String::from("314"),
            t: 2,
            url: String::from("https://ak.hycdn.cn/announce/Android/preannouncement/314_1635759750.html"),
        }
    }
}
