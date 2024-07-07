use crate::read_json;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_value;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub adaptive: bool,
    #[serde(rename = "enableServer")]
    pub enable_server: bool,
    #[serde(rename = "forceUpdateExcel")]
    pub force_update_excel: bool,
    pub gadget: bool,
    pub host: String,
    #[serde(rename = "maintenanceMsg")]
    pub maintenance_msg: String,
    pub mode: String,
    #[serde(rename = "noProxy")]
    pub no_proxy: bool,
    pub port: i64,
    #[serde(rename = "useSu")]
    pub use_su: bool,
}

impl ServerConfig {
    pub fn load() -> Result<Self> {
        let value = read_json("./config/server.json")["server"].clone();
        Ok(from_value(value)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RestorePrevState {
    is2: bool,
    #[serde(rename = "squadsAndFavs")]
    sq_n_fav: bool,
    ui: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(rename = "activityMaxStartTs")]
    pub act_max_start_ts: i64,
    #[serde(rename = "activityMinStartTs")]
    pub act_min_start_ts: i64,
    pub background: String,
    #[serde(rename = "fakeTime")]
    pub fake_time: i64,
    #[serde(rename = "forceEnableBattleReplay")]
    pub force_battle_replay: bool,
    #[serde(rename = "nickName")]
    pub name: String,
    #[serde(rename = "nickNumber")]
    pub number: String,
    #[serde(rename = "restorePreviousStates")]
    pub restore_prev_state: RestorePrevState,
    pub secretary: String,
    #[serde(rename = "secretarySkinId")]
    pub secretary_skin: String,
    pub theme: String,
    pub vision: bool,
}

impl UserConfig {
    pub fn load() -> Result<Self> {
        let value = read_json("./config/server.json")["userConfig"].clone();
        Ok(from_value(value)?)
    }
}
