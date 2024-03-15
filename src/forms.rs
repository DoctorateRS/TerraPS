use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub server: Server,
    pub assets: Assets,
    pub version: Version,
    pub remote: Remote,
    #[serde(rename = "networkConfig")]
    pub network_config: NetworkConfig,
    #[serde(rename = "crisisConfig")]
    pub crisis_config: CrisisConfig,
    #[serde(rename = "crisisV2Config")]
    pub crisis_v2config: CrisisConfig,
    #[serde(rename = "towerConfig")]
    pub tower_config: TowerConfig,
    #[serde(rename = "rlv2Config")]
    pub rlv2config: Rlv2Config,
    #[serde(rename = "battleReplayConfig")]
    pub battle_replay_config: BattleReplayConfig,
    #[serde(rename = "userConfig")]
    pub user_config: UserConfig,
    #[serde(rename = "charConfig")]
    pub char_config: CharConfig,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
    pub host: String,
    pub port: i64,
    #[serde(rename = "enableServer")]
    pub enable_server: bool,
    #[serde(rename = "maintenanceMsg")]
    pub maintenance_msg: String,
    pub mode: String,
    pub adaptive: bool,
    #[serde(rename = "noProxy")]
    pub no_proxy: bool,
    pub timeout: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Assets {
    #[serde(rename = "autoUpdate")]
    pub auto_update: bool,
    #[serde(rename = "downloadLocally")]
    pub download_locally: bool,
    #[serde(rename = "enableMods")]
    pub enable_mods: bool,
    #[serde(rename = "skipModCacheValidation")]
    pub skip_mod_cache_validation: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Version {
    pub android: Android,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Android {
    #[serde(rename = "resVersion")]
    pub res_version: String,
    #[serde(rename = "clientVersion")]
    pub client_version: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Remote {
    #[serde(rename = "enableGameBI")]
    pub enable_game_bi: bool,
    #[serde(rename = "enableSDKNetSecure")]
    pub enable_sdknet_secure: bool,
    #[serde(rename = "enableBestHttp")]
    pub enable_best_http: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct NetworkConfig {
    pub cn: Cn,
    pub global: Global,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Cn {
    pub sign: String,
    pub content: Content,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Content {
    #[serde(rename = "configVer")]
    pub config_ver: String,
    #[serde(rename = "funcVer")]
    pub func_ver: String,
    pub configs: Configs,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Configs {
    #[serde(rename = "V048")]
    pub v048: V048,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct V048 {
    #[serde(rename = "override")]
    pub override_field: bool,
    pub network: Network,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
    #[serde(rename = "pkgAd")]
    pub pkg_ad: Value,
    #[serde(rename = "pkgIOS")]
    pub pkg_ios: Value,
    pub secure: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Global {
    pub sign: String,
    pub content: Content2,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Content2 {
    #[serde(rename = "configVer")]
    pub config_ver: String,
    #[serde(rename = "funcVer")]
    pub func_ver: String,
    pub configs: Configs2,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Configs2 {
    #[serde(rename = "V029")]
    pub v029: V048,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CrisisConfig {
    #[serde(rename = "selectedCrisis")]
    pub selected_crisis: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TowerConfig {
    pub season: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Rlv2Config {
    #[serde(rename = "allChars")]
    pub all_chars: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct BattleReplayConfig {
    pub anonymous: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserConfig {
    #[serde(rename = "restorePreviousStates")]
    pub restore_previous_states: RestorePreviousStates,
    #[serde(rename = "activityMinStartTs")]
    pub activity_min_start_ts: i64,
    #[serde(rename = "activityMaxStartTs")]
    pub activity_max_start_ts: i64,
    pub secretary: String,
    #[serde(rename = "secretarySkinId")]
    pub secretary_skin_id: String,
    pub background: String,
    pub theme: String,
    #[serde(rename = "fakeTime")]
    pub fake_time: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RestorePreviousStates {
    pub is2: bool,
    #[serde(rename = "squadsAndFavs")]
    pub squads_and_favs: bool,
    pub ui: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CharConfig {
    #[serde(rename = "favorPoint")]
    pub favor_point: i64,
    #[serde(rename = "potentialRank")]
    pub potential_rank: i64,
    #[serde(rename = "mainSkillLvl")]
    pub main_skill_lvl: i64,
    pub level: i64,
    #[serde(rename = "evolvePhase")]
    pub evolve_phase: i64,
    #[serde(rename = "skillsSpecializeLevel")]
    pub skills_specialize_level: i64,
    #[serde(rename = "customUnitInfo")]
    pub custom_unit_info: CustomUnitInfo,
    #[serde(rename = "duplicateUnits")]
    pub duplicate_units: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomUnitInfo {}
