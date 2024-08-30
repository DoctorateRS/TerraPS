use serde::{Deserialize, Serialize};

use crate::{cnst::config::CONFIG_PATH, SERVER_CONFIG};
use common_utils::read_json;

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidNetwork {
    sign: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct V0531 {
    network: Network1,
    r#override: bool,
}
#[derive(Serialize, Deserialize)]
struct Configs1 {
    V053: V0531,
}
#[derive(Serialize, Deserialize)]
struct Content1 {
    configVer: String,
    configs: Configs1,
    funcVer: String,
}
#[derive(Serialize, Deserialize)]
struct Root1 {
    content: Content1,
    sign: String,
}

#[derive(Serialize, Deserialize)]
struct Network1 {
    an: String,
    #[serde(rename = "as")]
    ax: String,
    gs: String,
    hu: String,
    hv: String,
    of: String,
    pkgAd: (),
    pkgIOS: (),
    prean: String,
    rc: String,
    secure: bool,
    sl: String,
    u8: String,
}

impl ProdAndroidNetwork {
    pub fn load() -> Self {
        let mode = SERVER_CONFIG.mode.as_str();
        let host = SERVER_CONFIG.host.as_str();
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
