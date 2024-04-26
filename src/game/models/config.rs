use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub server: Server,
    pub assets: Assets,
    pub version: Version,
    pub version_global: VersionGlobal,
    pub remote: Remote,
    pub network_config: NetworkConfig,
    pub crisis_config: CrisisConfig,
    #[serde(rename = "crisisV2Config")]
    pub crisis_v2config: CrisisV2Config,
    pub tower_config: TowerConfig,
    #[serde(rename = "rlv2Config")]
    pub rlv2config: Rlv2Config,
    pub battle_replay_config: BattleReplayConfig,
    pub user_config: UserConfig,
    pub char_config: CharConfig,
    pub gacha: Gacha,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub host: String,
    pub port: i64,
    pub enable_server: bool,
    pub maintenance_msg: String,
    pub mode: String,
    pub adaptive: bool,
    pub no_proxy: bool,
    pub gadget: bool,
    pub use_su: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub auto_update: bool,
    pub download_locally: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub android: Android,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Android {
    pub res_version: String,
    pub client_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGlobal {
    pub android: Android2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Android2 {
    pub res_version: String,
    pub client_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remote {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
    pub cn: Cn,
    pub global: Global,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cn {
    pub sign: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub config_ver: String,
    pub func_ver: String,
    pub configs: Configs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configs {
    #[serde(rename = "V049")]
    pub v049: V049,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V049 {
    #[serde(rename = "override")]
    pub override_field: bool,
    pub network: Network,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub gs: String,
    #[serde(rename = "as")]
    pub as_field: String,
    pub u8: String,
    pub hu: String,
    pub hv: String,
    pub rc: String,
    pub an: String,
    pub prean: String,
    pub sl: String,
    pub of: String,
    pub pkg_ad: Value,
    #[serde(rename = "pkgIOS")]
    pub pkg_ios: Value,
    pub secure: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Global {
    pub sign: String,
    pub content: Content2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub config_ver: String,
    pub func_ver: String,
    pub configs: Configs2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configs2 {
    #[serde(rename = "V043")]
    pub v043: V043,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V043 {
    #[serde(rename = "override")]
    pub override_field: bool,
    pub network: Network2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network2 {
    pub gs: String,
    #[serde(rename = "as")]
    pub as_field: String,
    pub u8: String,
    pub hu: String,
    pub hv: String,
    pub rc: String,
    pub an: String,
    pub prean: String,
    pub sl: String,
    pub of: String,
    pub pkg_ad: Value,
    #[serde(rename = "pkgIOS")]
    pub pkg_ios: Value,
    pub secure: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisConfig {
    pub selected_crisis: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2Config {
    pub selected_crisis: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerConfig {
    pub season: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2Config {
    pub all_chars: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleReplayConfig {
    pub anonymous: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConfig {
    pub nick_name: String,
    pub nick_number: String,
    pub restore_previous_states: RestorePreviousStates,
    pub activity_min_start_ts: i64,
    pub activity_max_start_ts: i64,
    pub secretary: String,
    pub secretary_skin_id: String,
    pub background: String,
    pub theme: String,
    pub fake_time: i64,
    pub vision: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreviousStates {
    pub is2: bool,
    pub squads_and_favs: bool,
    pub ui: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharConfig {
    pub favor_point: i64,
    pub potential_rank: i64,
    pub main_skill_lvl: i64,
    pub level: i64,
    pub evolve_phase: i64,
    pub skills_specialize_level: i64,
    pub custom_unit_info: CustomUnitInfo,
    pub duplicate_units: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomUnitInfo {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gacha {
    #[serde(rename = "5rarity")]
    pub n5rarity: f64,
    #[serde(rename = "4rarity")]
    pub n4rarity: i64,
    #[serde(rename = "3rarity")]
    pub n3rarity: i64,
    #[serde(rename = "2rarity")]
    pub n2rarity: i64,
}
