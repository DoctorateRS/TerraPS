use crate::read_json;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_value;

#[derive(Serialize, Deserialize, Clone)]
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
        let value = read_json("./config/config.json")["server"].clone();
        Ok(from_value(value)?)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            adaptive: false,
            enable_server: true,
            force_update_excel: false,
            gadget: false,
            host: String::from("127.0.0.1"),
            maintenance_msg: String::from(""),
            mode: String::from("cn"),
            no_proxy: false,
            port: 8443,
            use_su: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RestorePrevState {
    is2: bool,
    #[serde(rename = "squadsAndFavs")]
    sq_n_fav: bool,
    ui: bool,
}

#[derive(Serialize, Deserialize, Clone)]
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
        let value = read_json("./config/config.json")["userConfig"].clone();
        Ok(from_value(value)?)
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            act_max_start_ts: 0,
            act_min_start_ts: 0,
            background: String::from("bg_sanrio_1"),
            fake_time: -1,
            force_battle_replay: false,
            name: String::from("Terra"),
            number: String::from("1111"),
            restore_prev_state: RestorePrevState {
                is2: false,
                sq_n_fav: false,
                ui: false,
            },
            secretary: String::from("char_377_gdglow"),
            secretary_skin: String::from("char_377_gdglow@sanrio#1"),
            theme: String::from("tm_rainbowsix_1"),
            vision: true,
        }
    }
}
