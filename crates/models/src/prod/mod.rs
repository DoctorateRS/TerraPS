pub mod network;

use std::collections::HashMap;

use common_utils::{read_json, ServerConfig};
use serde::{Deserialize, Serialize};
use serde_json::from_value;

pub use network::ProdAndroidNetwork;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProdAndroidVersion {
    pub res_version: String,
    pub client_version: String,
}

impl ProdAndroidVersion {
    pub fn load() -> Self {
        let mut versions = from_value::<HashMap<String, ProdAndroidVersion>>(read_json("./config/version.json")).unwrap();
        let server = ServerConfig::load().unwrap_or_default();

        versions.remove(&(if server.mode == "cn" { String::from("version") } else { String::from("versionGlobal") })).unwrap()
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProdAndroidRefresh {
    res_version: (),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Announce {
    announce_id: String,
    day: i64,
    group: String,
    is_web_url: bool,
    month: i64,
    title: String,
    web_url: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Extra {
    enable: bool,
    name: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementMeta {
    announce_list: Vec<Announce>,
    extra: Extra,
    focus_announce_id: (),
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

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidRemote {}
