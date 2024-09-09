use std::{collections::HashMap, fs::File};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTable {
    pub basic_info: HashMap<String, BasicActivityInfo>,
    pub activity: HashMap<String, HashMap<String, Activity>>,
    pub car_data: CarData,
}

impl LoadTable for ActivityTable {
    type Err = Error;

    fn load() -> Result<Self> {
        Ok(from_reader(File::open("../../data/excel/activity_table.json")?)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Activity {
    Act17Side(Box<Act17Side>),
    Others(OtherActs),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OtherActs {
    #[serde(default)]
    pub zone_list: Option<Vec<Zone>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    pub zone_id: String,
    pub zone_index: String,
    pub zone_name: String,
    pub zone_desc: String,
    pub item_drop_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CarData {
    pub car_dict: HashMap<String, CarComponent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CarComponent {
    pub pos_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17Side {
    pub place_data_map: HashMap<String, Act17SidePlace>,
    pub node_info_data_map: HashMap<String, Act17SideInfoNode>,
    pub choice_node_data_map: HashMap<String, Act17SideChoiceNode>,
    pub treasure_node_data_map: HashMap<String, Act17SideTreasureNode>,
    pub event_data_map: HashMap<String, Act17SideEvent>,
    pub story_node_data_map: HashMap<String, Act17SideStoryNode>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SidePlace {
    pub place_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideInfoNode {
    pub node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideStoryNode {
    pub story_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideChoiceNode {
    pub node_id: String,
    pub option_list: Vec<Act17SideChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideChoice {
    pub event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideTreasureNode {
    pub node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideEvent {
    pub event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17TechTree {
    pub default_branch_id: String,
}
