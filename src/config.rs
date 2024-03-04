use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub server: Server,
    pub version: Version,
    pub user_config: UserConfig,
    pub char_config: CharConfig,
    pub assist_units: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub android: Android,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Android {
    pub res_version: String,
    pub client_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserConfig {
    pub restore: Restore,
    pub secretary: String,
    pub secretary_skin: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Restore {
    pub integrated_strategies: bool,
    pub squad_and_favs: bool,
    pub ui: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharConfig {
    pub favor_point: u32,
    pub potential_rank: u8,
    pub skill_level: u8,
    pub level: i16,
    pub evolve_phase: i8,
    pub skills_specialize_level: u8,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 8443,
        }
    }
}
