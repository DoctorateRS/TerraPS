use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTable {
    pub basic_info: HashMap<String, BasicActivityInfo>,
    // TODO: Replace `Value` with a proper struct.
    pub activity: HashMap<String, Value>,
    pub car_data: CarData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicActivityInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub display_type: String,
    pub name: String,
    pub start_time: u64,
    pub end_time: u64,
    pub reward_end_time: u64,
    pub display_on_home: bool,
    pub has_stage: bool,
    pub template_shop_id: Option<String>,
    pub medal_group_id: Option<String>,
    pub ungrouped_medal_ids: Option<Vec<String>>,
    pub is_replicate: bool,
    pub need_fixed_sync: bool,
    pub trap_domain_id: Option<String>,
    pub rec_type: String,
    pub is_page_entry: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarData {
    pub car_dict: HashMap<String, Car>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub comp_id: String,
    pub sort_id: i32,
    #[serde(rename = "type")]
    pub component_type: String,
    pub pos_list: Vec<String>,
    pub pos_id_dict: HashMap<String, Vec<String>>,
    pub name: String,
    pub icon: String,
    pub show_scores: u16,
    pub item_usage: String,
    pub item_desc: String,
    pub item_obtain: String,
    pub rarity: u8,
    pub detail_desc: String,
    pub price: u16,
    pub special_obtain: String,
    pub obtain_in_random: bool,
    pub additive_color: Option<String>,
}
