mod network;

use common_utils::read_json;
use serde::{Deserialize, Serialize};
use serde_json::from_value;

use crate::{cnst::config::CONFIG_PATH, SERVER_CONFIG};
pub use network::ProdAndroidNetwork;

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

#[derive(Serialize, Deserialize, Default)]
pub struct ProdAndroidRefresh {
    #[serde(rename = "resVersion")]
    res_ver: (),
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

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidRemote {}
