use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::NullObj;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    pub assist: Vec<i16>,
    pub room_slots: HashMap<String, RoomSlot>,
    // [TODO] This is a placeholder for now
    pub rooms: HashMap<String, NullObj>,
    pub chars: HashMap<String, BuildingChar>,
    pub furniture: HashMap<String, Furniture>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSlot {
    pub char_inst_ids: Vec<i16>,
    pub complete_construct_time: i64,
    pub level: i16,
    pub room_id: String,
    pub state: i16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingChar {
    pub char_id: String,
    pub last_ap_add_time: u64,
    pub ap: u64,
    pub room_slot_id: String,
    pub index: i16,
    pub change_scale: i16,
    pub bubble: BuildingCharBubbleContainer,
    pub work_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingCharBubbleContainer {
    pub normal: BuildingCharBubble,
    pub assist: BuildingCharBubble,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingCharBubble {
    pub add: i16,
    pub ts: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Furniture {
    pub count: u32,
    pub in_use: u32,
}
