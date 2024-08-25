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
        pub zones: HashMap<String, Zone>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Zone {
        pub nodes: Vec<Node>,
        pub lines: Vec<Line>,
    }

#[derive(Deserialize, Serialize, Debug)]
    pub struct Node {
        pub id: String,
        pub node_type: String,
        pub coords: Coords,
        pub next_nodes: Vec<String>,
        pub content: NodeContent,
    }

#[derive(Deserialize, Serialize, Debug)]
    pub struct Coords {
        pub x: f32,
        pub y: f32,
    }

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
    pub enum NodeContent {
        #[serde(rename = "BATTLE")]
        Battle { stage_id: String },
        #[serde(rename = "SHOP")]
        Shop { shop_type: String },
        #[serde(rename = "INCIDENT")]
        Incident { incident_type: String },
        #[serde(rename = "REST")]
        Rest,
        #[serde(rename = "TREASURE")]
        Treasure,
    }

#[derive(Deserialize, Serialize, Debug)]
    pub struct Line {
        pub start: String,
        pub end: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Troop {
        pub chars: HashMap<String, Character>,
        pub expedition: Vec<Expedition>,
        pub expedition_return: Option<ExpeditionReturn>,
        pub has_expedition_return: bool,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Character {
        pub inst_id: u32,
        pub char_id: String,
        pub skin: String,
        pub level: u8,
        pub exp: u32,
        pub evolve_phase: u8,
        pub skills: Vec<Skill>,
        pub equip: HashMap<String, Equipment>,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Skill {
        pub id: String,
        pub level: u8,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Equipment {
        pub id: String,
        pub level: u8,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Expedition {
        pub char_inst_ids: Vec<u32>,
        pub duration: u32,
        pub start_time: u64,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct ExpeditionReturn {
        pub char_inst_ids: Vec<u32>,
        pub rewards: Vec<Reward>,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Reward {
        pub id: String,
        pub count: u32,
    }


    #[derive(Deserialize, Serialize, Debug)]
    pub struct Inventory {
        pub relic: HashMap<String, InventoryItem>,
        pub recruit: HashMap<String, InventoryItem>,
        pub trap: Option<Trap>,
        pub consumable: HashMap<String, InventoryItem>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct InventoryItem {
        pub count: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Trap {
        pub id: String,
        pub count: u32,
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
    pub struct Predefined {
        pub chars: Vec<String>,
        pub relics: Vec<String>,
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
    pub struct Capsule {
        pub id: String,
        pub count: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SquadBuff {
        pub id: String,
        pub cnt: u32,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Record {
        pub brief: Option<Brief>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Brief {
        pub start_ts: u64,
        pub end_ts: u64,
        pub player_name: String,
        pub doc_name: String,
        pub theme: String,
        pub mode: String,
        pub ending: String,
        pub rogue_like_is_victory: bool,
        pub squad: Vec<BriefCharacter>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct BriefCharacter {
        pub char_id: String,
        pub skin: String,
        pub level: u8,
        pub evolve_phase: u8,
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
        pub chaos: HashMap<String, CollectItem>,
        pub totem: TotemCollect,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CollectItem {
        pub state: u8,
        pub progress: Option<serde_json::Value>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct TotemCollect {
        pub totem: HashMap<String, CollectItem>,
        pub affix: HashMap<String, CollectItem>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Mission {
        pub update_id: String,
        pub refresh: u32,
        pub list: Vec<MissionItem>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct MissionItem {
        pub id: String,
        pub progress: u32,
        pub state: u8,
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
        pub collect: ChallengeCollect,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ChallengeCollect {
        pub explore_tool: HashMap<String, CollectItem>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MonthTeam {
        pub valid: Vec<String>,
        pub reward: HashMap<String, u8>,
        pub mission: HashMap<String, MonthTeamMission>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct MonthTeamMission {
        pub state: u8,
        pub current: u32,
        pub target: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct OuterRecord {
        pub last: u64,
        pub mode_cnt: HashMap<String, u32>,
        pub ending_cnt: HashMap<String, u32>,
        pub stage_cnt: HashMap<String, u32>,
        pub band_cnt: HashMap<String, HashMap<String, u32>>,
        pub band_grade: HashMap<String, HashMap<String, u8>>,
    }
}