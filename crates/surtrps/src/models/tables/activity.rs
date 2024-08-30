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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionData {
    pub progress_up_limit: u32,
    pub id: String,
    pub sort_id: u32,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub item_bg_type: String,
    pub pre_mission_ids: Option<Vec<String>>,
    pub template: String,
    pub template_type: String,
    pub param: Vec<String>,
    pub unlock_condition: Option<String>,
    pub unlock_param: Option<Vec<String>>,
    pub mission_group: String,
    pub to_page: Option<String>,
    pub periodical_point: u32,
    pub rewards: Vec<Reward>,
    pub back_image_path: Option<String>,
    pub fold_id: Option<String>,
    pub have_sub_mission_to_unlock: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    #[serde(rename = "type")]
    pub ty: String,
    pub id: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringRes {
    pub text_progress_format_can_claim: String,
    pub text_progress_format_cannot_claim: String,
    pub text_detail_color_can_claim: String,
    pub text_detail_color_cannot_claim: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTrap {
    pub template_traps: HashMap<String, TemplateTrap>,
    pub trap_const_data: TrapConstData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateTrap {
    pub trap_id: String,
    pub sort_id: u32,
    pub trap_name: String,
    pub trap_desc: String,
    pub trap_text: String,
    pub trap_task_id: String,
    pub trap_unlock_desc: String,
    pub trap_buff_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapConstData {
    pub stage_unlock_trap_desc: String,
    pub trap_maximum: u32,
    pub stage_can_not_use_trap: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapMissions {
    pub trap_missions: HashMap<String, TrapMission>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapMission {
    pub id: String,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub rewards: Vec<Reward>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneData {
    pub id: String,
    pub points: f32,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub key: String,
    pub selector: Selector,
    pub blackboard: Vec<Blackboard>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    pub profession_mask: u32,
    pub buildable_mask: String,
    pub player_side_mask: String,
    pub char_id_filter: Option<Vec<String>>,
    pub enemy_id_filter: Option<Vec<String>>,
    pub enemy_id_exclude_filter: Option<Vec<String>>,
    pub enemy_level_type_filter: Option<Vec<String>>,
    pub skill_id_filter: Option<Vec<String>>,
    pub tile_key_filter: Option<Vec<String>>,
    pub group_tag_filter: Option<Vec<String>>,
    pub filter_tag_filter: Option<Vec<String>>,
    pub filter_tag_exclude_filter: Option<Vec<String>>,
    pub sub_profession_exclude_filter: Option<Vec<String>>,
    pub map_tag_filter: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blackboard {
    pub key: String,
    pub value: f32,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTemplateMissionStyle {
    pub big_reward_type: String,
    pub big_reward_param_list: Vec<String>,
    pub is_mission_list_common_type: bool,
    pub is_mission_item_common_type: bool,
    pub mission_item_main_color: String,
    pub is_mission_item_complete_use_main_color: bool,
    pub mission_item_complete_color: Option<String>,
    pub is_mission_reward_item_common_type: bool,
    pub is_claim_all_btn_common_type: bool,
    pub claim_all_btn_main_color: String,
    pub claim_all_btn_tips: String,
    pub title_type: String,
    pub coin_type: String,
    pub coin_back_color: String,
}
    


