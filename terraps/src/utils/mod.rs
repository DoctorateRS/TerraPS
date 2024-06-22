use std::str::FromStr;

use crate::constants::config::CONFIG_JSON_PATH;

use self::{
    fmt::{ccv2_fmt, cfg_fmt, excel_fmt},
    update::{excel_update, update_cn_config, update_gacha},
};

use common_utils::{read_json, write_json};

use anyhow::Result;
use serde_json::{json, Value};
use update::{update_event, Mode};

pub mod battle_data;
pub mod battle_replay;
pub mod comp;
pub mod crypto;
pub mod fmt;
pub mod game;
pub mod json;
pub mod mail;
pub mod random;
pub mod rlv2;
pub mod server;
pub mod time;
pub mod update;

pub fn enumerate<T: IntoIterator>(a: T) -> Vec<(usize, T::Item)> {
    a.into_iter().enumerate().collect()
}

pub fn get_nickname_config() -> (String, String) {
    let config = read_json(CONFIG_JSON_PATH);
    let mut mut_conf = config.clone();
    let nick_name = match config["userConfig"]["nickName"].as_str() {
        Some(nick_name) => nick_name,
        None => {
            mut_conf["userConfig"]["nickName"] = json!("Terra");
            write_json(CONFIG_JSON_PATH, &mut_conf);
            "Terra"
        }
    };
    let nick_id = match config["userConfig"]["nickNumber"].as_str() {
        Some(nick_name) => nick_name,
        None => {
            mut_conf["userConfig"]["nickNumber"] = json!("1111");
            write_json(CONFIG_JSON_PATH, &mut_conf);
            "1111"
        }
    };
    (nick_name.to_string(), nick_id.to_string())
}

pub async fn upgrade() -> Result<()> {
    let config = read_json(CONFIG_JSON_PATH);
    let update_required = update_cn_config().await?;

    let force_update = config["server"]["forceUpdateExcel"].as_bool().unwrap_or(false);
    let mode = Mode::from_str(config["server"]["mode"].as_str().unwrap_or("cn")).unwrap();

    if update_required || force_update {
        excel_update(mode).await?;
        update_gacha().await?;
        excel_fmt()?;
        cfg_fmt()?;
        ccv2_fmt()?;
    }

    update_event()?;
    Ok(())
}

pub fn str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::from(""),
    }
}
