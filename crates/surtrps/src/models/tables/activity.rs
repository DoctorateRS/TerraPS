use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTable {
    pub basic_info: HashMap<String, BasicActivityInfo>,
    pub home_act_config: HashMap<String, HomeActConf>,
    pub zone_to_activity: HashMap<String, String>,
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
pub struct HomeActConf {
    pub act_id: String,
    pub is_popup_after_checkin: bool,
    pub show_top_bar_menu: bool,
    pub act_top_bar_color: Option<String>,
    pub act_top_bar_text: Option<String>,
}
