use crate::constants::{config::CONFIG_JSON_PATH, url::*};

use super::json::{read_json, write_json};

use anyhow::Result;
use reqwest::get;
use serde_json::{from_str, json, Value};

const VER_AK_CONF: &str = "https://ak-conf.hypergryph.com/config/prod/official/Android/version";
const NW_AK_CONF: &str = "https://ak-conf.hypergryph.com/config/prod/official/network_config";

pub async fn update_config() -> Result<()> {
    let stcf = read_json(CONFIG_JSON_PATH);
    let mut config = stcf.clone();

    let old_res_version = &stcf["version"]["android"]["resVersion"];
    let old_client_version = &stcf["version"]["android"]["clientVersion"];
    let old_func_ver = &stcf["networkConfig"]["cn"]["content"]["funcVer"];

    let new_ver_config = get(VER_AK_CONF).await?.json::<Value>().await?;
    let new_nw_config = get(NW_AK_CONF).await?.json::<Value>().await?;

    if old_res_version != &new_ver_config["resVersion"] {
        config["version"]["android"]["resVersion"] = json!(new_ver_config["resVersion"]);
    }
    if old_client_version != &new_ver_config["clientVersion"] {
        config["version"]["android"]["clientVersion"] = json!(new_ver_config["clientVersion"]);
    }

    let content = from_str::<Value>(new_nw_config["content"].as_str().unwrap())?;
    let func_ver = &content["funcVer"];
    if old_func_ver != func_ver {
        config["networkConfig"]["cn"]["content"]["funcVer"] = json!(func_ver);
        config["networkConfig"]["cn"]["content"]["configs"][func_ver.as_str().unwrap()] =
            config["networkConfig"]["cn"]["content"]["configs"][old_func_ver.as_str().unwrap()].clone();
        config["networkConfig"]["cn"]["content"]["configs"]
            .as_object_mut()
            .unwrap()
            .remove(old_func_ver.as_str().unwrap());
    }

    write_json(CONFIG_JSON_PATH, config);

    Ok(())
}

pub async fn excel_update() {}
