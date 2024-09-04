use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::curprop::Rlv2CurrentPlayerProperty;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2Current {
    pub player: Rlv2CurrentPlayer,
    pub map: Rlv2CurrentMap,
    pub troop: Rlv2CurrentTroop,
    pub inventory: Rlv2CurrentInventory,
    pub game: Rlv2CurrentGame,
    pub buff: Rlv2CurrentBuff,
    pub record: Rlv2CurrentRecord,
    pub module: HashMap<String, Rlv2CurrentModule>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2CurrentPlayer {
    pub state: String,
    pub property: Rlv2CurrentPlayerProperty,
    pub cursor: Rlv2CurrentPlayerCursor,
    pub trace: Vec<Rlv2CurrentPlayerCursor>,
    pub pending: Vec<Rlv2CurrentPlayerPendingContent>,
    pub status: Rlv2CurrentPlayerStatus,
    pub to_ending: String,
    pub chg_ending: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentPlayerCursor {
    pub zone: u32,
    pub position: Option<Rlv2CurrentPlayerCursorPos>,
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentPlayerCursorPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2CurrentPlayerPendingContentContainer {
    pub index: String,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rlv2CurrentPlayerPendingContent {
    InitRelic {},
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentPlayerStatus {
    pub bank_put: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Rlv2CurrentMap {
    pub zones: HashMap<String, Rlv2CurrentMapZone>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentMapZone {
    pub nodes: Vec<Rlv2CurrentMapZoneNode>,
    pub lines: Vec<Rlv2CurrentMapZoneLine>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentMapZoneNode {
    pub id: String,
    pub node_type: String,
    pub coords: Rlv2CurrentMapZoneNodeCoords,
    pub next_nodes: Vec<String>,
    pub content: Rlv2CurrentMapZoneNodeContent,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentMapZoneNodeCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum Rlv2CurrentMapZoneNodeContent {
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
pub struct Rlv2CurrentMapZoneLine {
    pub start: String,
    pub end: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroop {
    pub chars: HashMap<String, Rlv2CurrentTroopCharacter>,
    pub expedition: Vec<Rlv2CurrentTroopExpedition>,
    pub expedition_return: Option<Rlv2CurrentTroopExpeditionReturn>,
    pub has_expedition_return: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopCharacter {
    pub inst_id: u32,
    pub char_id: String,
    pub skin: String,
    pub level: u8,
    pub exp: u32,
    pub evolve_phase: u8,
    pub skills: Vec<Rlv2CurrentTroopCharacterSkill>,
    pub equip: HashMap<String, Rlv2CurrentTroopCharacterEquipment>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopCharacterSkill {
    pub id: String,
    pub level: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopCharacterEquipment {
    pub id: String,
    pub level: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopExpedition {
    pub char_inst_ids: Vec<u32>,
    pub duration: u32,
    pub start_time: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopExpeditionReturn {
    pub char_inst_ids: Vec<u32>,
    pub rewards: Vec<Rlv2CurrentTroopExpeditionReturnReward>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentTroopExpeditionReturnReward {
    pub id: String,
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentInventory {
    pub relic: HashMap<String, Rlv2CurrentInventoryItem>,
    pub recruit: HashMap<String, Rlv2CurrentInventoryItem>,
    pub trap: Option<Rlv2CurrentInventoryTrap>,
    pub consumable: HashMap<String, Rlv2CurrentInventoryItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentInventoryItem {
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentInventoryTrap {
    pub id: String,
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2CurrentGame {
    pub mode: String,
    pub predefined: Option<Rlv2CurrentGamePredefined>,
    pub theme: String,
    pub outer: Rlv2CurrentGameOuter,
    pub start: i64,
    pub mode_grade: u32,
    pub equivalent_grade: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentGamePredefined {
    pub chars: Vec<String>,
    pub relics: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentGameOuter {
    pub support: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentBuff {
    pub tmp_hp: u32,
    pub capsule: Option<Rlv2CurrentBuffCapsule>,
    pub squad_buff: Vec<Rlv2CurrentBuffSquadBuff>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentBuffCapsule {
    pub id: String,
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentBuffSquadBuff {
    pub id: String,
    pub cnt: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentRecord {
    pub brief: Option<Rlv2CurrentRecordBrief>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentRecordBrief {
    pub start_ts: u64,
    pub end_ts: u64,
    pub player_name: String,
    pub doc_name: String,
    pub theme: String,
    pub mode: String,
    pub ending: String,
    pub rogue_like_is_victory: bool,
    pub squad: Vec<Rlv2CurrentRecordBriefCharacter>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentRecordBriefCharacter {
    pub char_id: String,
    pub skin: String,
    pub level: u8,
    pub evolve_phase: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2CurrentModule {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2Outer {
    pub bank: Rlv2OuterBank,
    pub bp: Rlv2OuterBp,
    pub buff: Rlv2OuterBuff,
    pub collect: Rlv2OuterCollect,
    pub mission: Rlv2OuterMission,
    pub challenge: Rlv2OuterChallenge,
    pub month_team: Rlv2OuterMonthTeam,
    pub record: Rlv2OuterRecord,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterBank {
    pub show: bool,
    pub current: u32,
    pub record: u32,
    pub reward: HashMap<String, u8>,
    pub total_put: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterBp {
    pub point: u32,
    pub reward: HashMap<String, u8>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2OuterBuff {
    pub point_owned: u32,
    pub point_cost: u32,
    pub unlocked: HashMap<String, u8>,
    pub score: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterCollect {
    pub relic: HashMap<String, Rlv2OuterCollectItem>,
    pub capsule: HashMap<String, Rlv2OuterCollectItem>,
    pub active_tool: HashMap<String, Rlv2OuterCollectItem>,
    pub bgm: HashMap<String, u8>,
    pub pic: HashMap<String, u8>,
    pub chat: HashMap<String, u8>,
    pub band: HashMap<String, Rlv2OuterCollectItem>,
    pub buff: HashMap<String, Rlv2OuterCollectItem>,
    pub end_book: HashMap<String, Rlv2OuterCollectItem>,
    pub mode: HashMap<String, Rlv2OuterCollectItem>,
    pub recruit_set: HashMap<String, Rlv2OuterCollectItem>,
    pub mode_grade: HashMap<String, HashMap<String, Rlv2OuterCollectItem>>,
    pub chaos: HashMap<String, Rlv2OuterCollectItem>,
    pub totem: Rlv2OuterCollectTotem,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterCollectItem {
    pub state: u8,
    pub progress: Option<Rlv2OuterCollectItemProgress>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterCollectItemProgress {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterCollectTotem {
    pub totem: HashMap<String, Rlv2OuterCollectItem>,
    pub affix: HashMap<String, Rlv2OuterCollectItem>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2OuterMission {
    pub update_id: String,
    pub refresh: u32,
    pub list: Vec<Rlv2OuterMissionItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterMissionItem {
    pub id: String,
    pub progress: u32,
    pub state: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterChallenge {
    pub reward: HashMap<String, u8>,
    pub grade: HashMap<String, u8>,
    pub collect: Rlv2OuterChallengeCollect,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterChallengeCollect {
    pub explore_tool: HashMap<String, Rlv2OuterCollectItem>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2OuterMonthTeam {
    pub valid: Vec<String>,
    pub reward: HashMap<String, u8>,
    pub mission: HashMap<String, Rlv2OuterMonthTeamMission>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterMonthTeamMission {
    pub state: u8,
    pub current: u32,
    pub target: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rlv2OuterRecord {
    pub last: u64,
    pub mode_cnt: HashMap<String, u32>,
    pub ending_cnt: HashMap<String, u32>,
    pub stage_cnt: HashMap<String, u32>,
    pub band_cnt: HashMap<String, HashMap<String, u32>>,
    pub band_grade: HashMap<String, HashMap<String, u8>>,
}
