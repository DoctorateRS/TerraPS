use std::str::FromStr;

use self::{
    fmt::{ccv2_fmt, cfg_fmt, excel_fmt},
    update::{excel_update, update_cn_config, update_gacha},
};

use common_utils::{ServerConfig, UserConfig};

use anyhow::Result;
use serde_json::Value;
use update::{update_event, Mode};

pub mod battle_data;
pub mod battle_replay;
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
    let config = UserConfig::load().unwrap_or_default();
    let nick_name = &config.name;
    let nick_id = &config.number;
    (nick_name.into(), nick_id.into())
}

pub async fn upgrade() -> Result<()> {
    let config = ServerConfig::load()?;
    let update_required = update_cn_config().await?;

    let force_update = config.force_update_excel;
    let mode = Mode::from_str(&config.mode).unwrap();

    if update_required || force_update {
        excel_update(mode).await?;
        update_gacha().await?;
        update_event()?;
        excel_fmt()?;
        cfg_fmt()?;
        ccv2_fmt()?;
    }

    Ok(())
}

pub fn str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::from(""),
    }
}
