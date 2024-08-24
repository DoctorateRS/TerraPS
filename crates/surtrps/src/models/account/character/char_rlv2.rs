use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod rlv2 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Rlv2 {
        pub current: Current,
        pub outer: HashMap<String, Outer>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Current {
        pub player: Player,
        pub map: Map,
        pub troop: Troop,
        pub inventory: Inventory,
        pub game: Game,
        pub buff: Buff,
        pub record: Record,
        pub module: HashMap<String, serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Player {
        pub state: String,
        pub property: Property,
        pub cursor: Cursor,
        pub trace: Vec<serde_json::Value>,
        pub pending: Vec<serde_json::Value>,
        pub status: Status,
        pub to_ending: String,
        pub chg_ending: bool,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Property {
        pub exp: u32,
        pub level: u8,
        pub max_level: u8,
        pub hp: Hp,
        pub gold: u32,
        pub shield: u32,
        pub capacity: u32,
        pub population: Population,
        pub con_perfect_battle: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Hp {
        pub current: u32,
        pub max: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Population {
        pub cost: u32,
        pub max: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Cursor {
        pub zone: u32,
        pub position: Option<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Status {
        pub bank_put: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Map {
        pub zones: HashMap<String, serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Troop {
        pub chars: HashMap<String, serde_json::Value>,
        pub expedition: Vec<serde_json::Value>,
        pub expedition_return: Option<serde_json::Value>,
        pub has_expedition_return: bool,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Inventory {
        pub relic: HashMap<String, serde_json::Value>,
        pub recruit: HashMap<String, serde_json::Value>,
        pub trap: Option<serde_json::Value>,
        pub consumable: HashMap<String, serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Game {
        pub mode: String,
        pub predefined: Option<serde_json::Value>,
        pub theme: String,
        pub outer: GameOuter,
        pub start: i64,
        pub mode_grade: u32,
        pub equivalent_grade: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct GameOuter {
        pub support: bool,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Buff {
        pub tmp_hp: u32,
        pub capsule: Option<serde_json::Value>,
        pub squad_buff: Vec<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Record {
        pub brief: Option<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Outer {
        pub bank: Bank,
        pub bp: Bp,
        pub buff: OuterBuff,
        pub collect: Collect,
        pub mission: Mission,
        pub challenge: Challenge,
        pub month_team: MonthTeam,
        pub record: OuterRecord,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Bank {
        pub show: bool,
        pub current: u32,
        pub record: u32,
        pub reward: HashMap<String, u8>,
        pub total_put: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Bp {
        pub point: u32,
        pub reward: HashMap<String, u8>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct OuterBuff {
        pub point_owned: u32,
        pub point_cost: u32,
        pub unlocked: HashMap<String, u8>,
        pub score: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Collect {
        pub relic: HashMap<String, CollectItem>,
        pub capsule: HashMap<String, CollectItem>,
        pub active_tool: HashMap<String, CollectItem>,
        pub bgm: HashMap<String, u8>,
        pub pic: HashMap<String, u8>,
        pub chat: HashMap<String, u8>,
        pub band: HashMap<String, CollectItem>,
        pub buff: HashMap<String, CollectItem>,
        pub end_book: HashMap<String, CollectItem>,
        pub mode: HashMap<String, CollectItem>,
        pub recruit_set: HashMap<String, CollectItem>,
        pub mode_grade: HashMap<String, HashMap<String, CollectItem>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CollectItem {
        pub state: u8,
        pub progress: Option<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Mission {
        pub update_id: String,
        pub refresh: u32,
        pub list: Vec<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Challenge {
        pub reward: HashMap<String, u8>,
        pub grade: HashMap<String, u8>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MonthTeam {
        pub valid: Vec<String>,
        pub reward: HashMap<String, u8>,
        pub mission: HashMap<String, serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct OuterRecord {
        pub last: u64,
        pub mode_cnt: HashMap<String, serde_json::Value>,
        pub ending_cnt: HashMap<String, serde_json::Value>,
        pub stage_cnt: HashMap<String, u32>,
        pub band_cnt: HashMap<String, HashMap<String, u32>>,
    }
}