use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTable {
    pub basic_info: HashMap<String, BasicActivityInfo>,
    pub home_act_config: HashMap<String, HomeActConf>,
    pub zone_to_activity: HashMap<String, String>,
    pub sync_points: SyncPoints,
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
    pub id: String,
    pub sort_id: i32,
    pub description: String,
    #[serde(rename = "type")]
    pub mission_type: String,
    pub item_bg_type: String,
    pub pre_mission_ids: Option<Vec<String>>,
    pub template: String,
    pub template_type: String,
    pub param: Vec<String>,
    pub unlock_condition: Option<String>,
    pub unlock_param: Option<Vec<String>>,
    pub mission_group: String,
    pub to_page: Option<String>,
    pub periodical_point: i32,
    pub rewards: Vec<Reward>,
    pub back_image_path: Option<String>,
    pub fold_id: Option<String>,
    pub have_sub_mission_to_unlock: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub count: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionGroup {
    pub id: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub group_type: String,
    pub pre_mission_group: Option<String>,
    pub period: Option<String>,
    pub rewards: Vec<Reward>,
    pub mission_ids: Vec<String>,
    pub start_ts: i64,
    pub end_ts: i64,
}


// replicateMission & activity stuff is currently empty and not implemented yet.


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraData {
    pub mainline_bp: HashMap<String, MainlineBp>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainlineBp {
    pub period_data_list: Vec<PeriodData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodData {
    pub period_id: String,
    pub start_ts: i64,
    pub end_ts: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItems {
    #[serde(flatten)]
    pub items: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct SyncPoints {
    #[serde(flatten)]
    pub activities: HashMap<String, Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynActs {
    #[serde(flatten)]
    pub activities: HashMap<String, PrayActivity>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrayActivity {
    pub rule1: String,
    pub rule2: String,
    pub slot_count: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageRewardsData {
    pub stage_rewards_dict: HashMap<String, HashMap<String, Vec<StageReward>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageReward {
    pub occ_percent: String,
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub drop_type: String,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActTheme {
    pub id: String,
    #[serde(rename = "type")]
    pub theme_type: String,
    pub func_id: String,
    pub end_ts: i64,
    pub sort_id: i32,
    pub item_id: String,
    pub time_nodes: Vec<TimeNode>,
    pub start_ts: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeNode {
    pub title: String,
    pub ts: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActFunData {
    pub stages: HashMap<String, Stage>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub stage_id: String,
    pub level_id: String,
    pub code: String,
    pub name: String,
    pub appearance_style: String,
    pub loading_pic_id: String,
    pub difficulty: String,
    pub unlock_condition: Vec<UnlockCondition>,
    pub stage_drop_info: Vec<StageDrop>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockCondition {
    pub stage_id: String,
    pub complete_state: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageDrop {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub drop_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarData {
    pub car_dict: HashMap<String, CarComponent>,
    pub rune_data_dict: HashMap<String, RuneData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarComponent {
    pub comp_id: String,
    pub sort_id: i32,
    #[serde(rename = "type")]
    pub component_type: String,
    pub pos_list: Vec<String>,
    pub pos_id_dict: HashMap<String, Vec<String>>,
    pub name: String,
    pub icon: String,
    pub show_scores: i32,
    pub item_usage: String,
    pub item_desc: String,
    pub item_obtain: String,
    pub rarity: i32,
    pub detail_desc: String,
    pub price: i32,
    pub special_obtain: String,
    pub obtain_in_random: bool,
    pub additive_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: String,
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
    pub profession_mask: String,
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
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartStages {
    pub cart_stages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstData {
    pub car_item_unlock_stage_id: String,
    pub car_item_unlock_desc: String,
    pub sp_level_unlock_item_cnt: i32,
    pub mile_stone_base_interval: i32,
    pub sp_stage_ids: Vec<String>,
    pub car_frame_default_color: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiracusaData {
    pub area_data_map: HashMap<String, AreaData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaData {
    pub area_id: String,
    pub area_name: String,
    pub area_sub_name: String,
    pub unlock_type: String,
    pub unlock_stage: Option<String>,
    pub area_icon_id: String,
    pub point_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KvSwitchData {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityKvSwitch>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityKvSwitch {
    pub kv_switch_info: HashMap<String, KvSwitchInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KvSwitchInfo {
    pub is_default: bool,
    pub display_time: i64,
    pub zone_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HiddenStage {
    pub stage_id: String,
    pub encoded_name: String,
    pub show_stage_id: String,
    pub reward_diamond: bool,
    pub missions: Vec<HiddenStageMission>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HiddenStageMission {
    pub unlock_stage_id: String,
    pub unlock_template: String,
    pub unlock_params: Option<Vec<String>>,
    pub mission_stage_id: String,
    pub unlocked_name: String,
    pub locked_name: String,
    pub lock_code: String,
    pub unlocked_des: String,
    pub template_desc: String,
    pub desc: String,
    pub riddle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionArchives {
    #[serde(flatten)]
    pub archives: HashMap<String, MissionArchive>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionArchive {
    pub topic_id: String,
    pub zones: Vec<String>,
    pub nodes: Vec<ArchiveNode>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveNode {
    pub node_id: String,
    pub title: String,
    pub unlock_desc: String,
    pub clips: Vec<ArchiveClip>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveClip {
    pub char_id: String,
    pub voice_id: String,
    pub index: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FifthAnnivExploreData {
    pub explore_group_data: HashMap<String, ExploreGroup>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExploreGroup {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub code: String,
    pub icon_id: String,
    pub initial_values: HashMap<String, i32>,
    pub heritage_value_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringRes {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityStringRes>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityStringRes {
    pub text_progress_format_can_claim: String,
    pub text_progress_format_cannot_claim: String,
    pub text_detail_color_can_claim: String,
    pub text_detail_color_cannot_claim: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTraps {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityTrapData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTrapData {
    pub template_traps: HashMap<String, TemplateTrap>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateTrap {
    pub trap_id: String,
    pub sort_id: i32,
    pub trap_name: String,
    pub trap_desc: String,
    pub trap_text: String,
    pub trap_task_id: String,
    pub trap_unlock_desc: String,
    pub trap_buff_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneDataDict {
    #[serde(flatten)]
    pub runes: HashMap<String, TrapRune>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapRune {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<TrapRuneEffect>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneEffect {
    pub key: String,
    pub selector: TrapRuneSelector,
    pub blackboard: Vec<TrapRuneBlackboard>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneSelector {
    pub profession_mask: i32,
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
pub struct TrapRuneBlackboard {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTemplateMissionStyles {
    #[serde(flatten)]
    pub styles: HashMap<String, TemplateMissionStyle>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMissionStyle {
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