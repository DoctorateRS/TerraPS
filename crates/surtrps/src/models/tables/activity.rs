use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

use crate::cnst::table_paths::ACTIVITY_TABLE_PATH;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTable {
    pub basic_info: HashMap<String, BasicActivityInfo>,
    pub activity: HashMap<String, Activity>,
    pub car_data: CarData,
    pub sync_points: SyncPoints,
}

impl ActivityTable {
    pub fn load() -> Self {
        &from_str(&{
            let mut ct = String::new();
            let mut f = File::open(ACTIVITY_TABLE_PATH).unwrap();
            f.read_to_string(&mut ct).unwrap();
            ct
        })
        .unwrap()
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

// Start of Activity Structs

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityData {
    pub activity: HashMap<String, HashMap<String, Activity>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CheckinOnlyActivities {
    pub checkin_only: HashMap<String, CheckinActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckinActivity {
    pub check_in_list: HashMap<String, CheckInDay>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInDay {
    pub item_list: Vec<CheckInItem>,
    pub order: i32,
    pub color: i32,
    pub key_item: i32,
    pub show_item_order: i32,
    pub is_dyn_item: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CheckinAllPlayerActivities {
    pub checkin_all_player: HashMap<String, CheckinAllPlayerActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckinAllPlayerActivity {
    pub check_in_list: HashMap<String, CheckInAllPlayerDay>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
    pub pub_bhvs: HashMap<String, PubBehavior>,
    pub personal_bhvs: HashMap<String, PersonalBehavior>,
    pub const_data: ConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBehavior {
    pub sort_id: u16,
    pub personal_behavior_id: String,
    pub display_order: i32,
    pub require_repeat_completion: bool,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CheckinVsActivities {
    pub checkin_vs: HashMap<String, CheckinVsActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckinVsActivity {
    pub check_in_dict: HashMap<String, CheckInVsDay>,
    pub vote_taste_list: Vec<VoteTaste>,
    pub taste_info_dict: HashMap<String, TasteInfo>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
    pub versus_total_days: i32,
    pub rule_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInVsDay {
    pub reward_list: Vec<CheckInVsReward>,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInVsReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VoteTaste {
    pub pl_sweet_num: i32,
    pub pl_salty_num: i32,
    pub pl_taste: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TasteInfo {
    pub pl_taste: i32,
    pub taste_type: TasteType,
    pub taste_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TasteReward {
    pub taste_type: TasteType,
    pub reward_item: RewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TasteType {
    Sweet,
    Salt,
    Draw,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct3d0Activities {
    pub type_act3d0: HashMap<String, Act3d0Activity>,
    pub limited_pool_list: HashMap<String, LimitedPool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act3d0Activity {
    pub camp_basic_info: HashMap<String, CampBasicInfo>,
    pub limited_pool_list: HashMap<String, LimitedPool>,
    pub infinite_pool_list: HashMap<String, Value>,
    pub infinite_percent: Option<f64>,
    pub camp_item_map_info: HashMap<String, CampItemMap>,
    pub clue_info: HashMap<String, ClueInfo>,
    pub mile_stone_token_id: String,
    pub coin_token_id: String,
    pub et_token_id: String,
    pub gacha_box_info: Vec<GachaBoxInfo>,
    pub camp_info: Option<Value>,
    pub zone_desc: HashMap<String, ZoneDesc>,
    pub favor_up_list: HashMap<String, FavorUp>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CampBasicInfo {
    pub camp_id: String,
    pub camp_name: String,
    pub camp_desc: String,
    pub reward_desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LimitedPool {
    pub pool_id: String,
    pub pool_item_info: Vec<PoolItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PoolItem {
    pub good_id: String,
    pub item_info: ItemInfo,
    pub good_type: String,
    pub per_count: i32,
    pub total_count: i32,
    pub weight: i32,
    #[serde(rename = "type")]
    pub item_type: String,
    pub order_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CampItemMap {
    pub good_id: String,
    pub item_dict: HashMap<String, CampItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CampItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClueInfo {
    pub item_id: String,
    pub camp_id: String,
    pub order_id: i32,
    pub image_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GachaBoxInfo {
    pub gacha_box_id: String,
    pub box_type: String,
    pub key_good_id: String,
    pub token_id: TokenId,
    pub token_num_once: i32,
    pub unlock_img: Option<String>,
    pub next_gacha_box_info_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenId {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneDesc {
    pub zone_id: String,
    pub locked_text: Option<String>,
    pub display_start_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FavorUp {
    pub char_id: String,
    pub display_start_time: u64,
    pub display_end_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct4d0Activities {
    pub type_act4d0: HashMap<String, Act4d0Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act4d0Activity {
    pub mile_stone_item_list: Vec<MileStoneItem>,
    pub mile_stone_story_list: Vec<MileStoneStory>,
    pub story_info_list: Vec<StoryInfo>,
    pub stage_info: Vec<StageInfo>,
    pub token_item: TokenItem,
    pub char_stone_id: String,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
    pub extra_drop_zones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MileStoneItem {
    pub mile_stone_id: String,
    pub order_id: i32,
    pub token_num: i32,
    pub item: RewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MileStoneStory {
    pub mile_stone_id: String,
    pub order_id: i32,
    pub token_num: i32,
    pub story_key: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StoryInfo {
    pub story_key: String,
    pub story_id: String,
    pub story_sort: String,
    pub story_name: String,
    pub lock_desc: String,
    pub story_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageInfo {
    pub stage_key: String,
    pub zone_id: String,
    pub stage_id: String,
    pub unlock_desc: String,
    pub lock_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct5d0Activities {
    pub type_act5d0: HashMap<String, Act5d0Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act5d0Activity {
    pub mile_stone_info: Vec<MileStoneInfo>,
    pub mile_stone_token_id: String,
    pub zone_desc: HashMap<String, ZoneDesc>,
    pub mission_extra_list: HashMap<String, MissionExtra>,
    pub sp_reward: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MileStoneInfo {
    pub mile_stone_id: String,
    pub order_id: i32,
    pub token_num: i32,
    pub mile_stone_type: String,
    pub normal_item: NormalItem,
    #[serde(rename = "IsBonus")]
    pub is_bonus: i32,
    pub item: Item,
    pub is_precious: bool,
    pub mile_stone_stage: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NormalItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionExtra {
    pub difficult_level: i32,
    pub level_desc: String,
    pub sort_id: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct5d1Activities {
    pub type_act5d1: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CollectionActivities {
    pub collection: HashMap<String, CollectionActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionActivity {
    pub collections: Vec<Collection>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: i32,
    pub item_type: String,
    pub item_id: String,
    pub item_cnt: i32,
    pub point_id: String,
    pub point_cnt: i32,
    pub is_bonus: bool,
    pub png_name: Option<String>,
    pub png_sort: i32,
    pub is_show: bool,
    pub show_in_list: bool,
    pub show_icon_bg: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct9d0Activities {
    pub type_act9d0: HashMap<String, Act9d0Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act9d0Activity {
    pub token_item_id: String,
    pub zone_desc_list: HashMap<String, ZoneDesc>,
    pub favor_up_list: HashMap<String, Value>,
    pub sub_mission_info: Option<Value>,
    pub has_sub_mission: bool,
    pub ap_supply_out_of_date_dict: HashMap<String, Value>,
    pub news_info_list: Option<Value>,
    pub news_server_info_list: Option<Value>,
    pub misc_hub: HashMap<String, Value>,
    pub const_data: ConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct12SideActivities {
    pub type_act12side: HashMap<String, Act12SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act12SideActivity {
    pub const_data: Act12SideConstData,
    pub zone_addition_data_list: Vec<Act12ZoneAdditionData>,
    pub mission_desc_list: HashMap<String, MissionDesc>,
    pub mile_stone_info_list: Vec<MileStoneInfo>,
    pub photo_list: HashMap<String, Photo>,
    pub recycle_dialog_dict: HashMap<String, Vec<RecycleDialog>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act12SideConstData {
    pub recycle_reward_threshold: i32,
    pub charm_repo_unlock_stage_id: String,
    pub recycle_low_threshold: i32,
    pub recycle_medium_threshold: i32,
    pub recycle_high_threshold: i32,
    pub auto_get_charm_id: String,
    pub fog_stage_id: String,
    pub fog_unlock_stage_id: String,
    pub fog_unlock_ts: u64,
    pub fog_unlock_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act12ZoneAdditionData {
    pub zone_id: String,
    pub unlock_text: String,
    pub zone_class: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionDesc {
    pub zone_class: String,
    pub special_mission_desc: String,
    pub need_lock: bool,
    pub unlock_hint: Option<String>,
    pub unlock_stage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub pic_id: String,
    pub pic_name: String,
    pub mile_stone_id: String,
    pub pic_desc: String,
    pub jump_stage_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecycleDialog {
    pub dialog_type: String,
    pub dialog: String,
    pub dialog_express: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct13SideActivities {
    pub type_act13side: HashMap<String, Act13SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act13SideActivity {
    pub const_data: Act13SideConstData,
    pub org_data_map: HashMap<String, OrgData>,
    pub long_term_mission_data_map: HashMap<String, LongTermMissionData>,
    pub daily_mission_data_list: Vec<DailyMissionData>,
    pub archive_item_unlock_data: HashMap<String, ArchiveItemUnlockData>,
    pub hidden_area_data: HashMap<String, HiddenAreaData>,
    #[serde(rename = "zoneAddtionDataMap")]
    pub zone_addition_data_map: HashMap<String, Act13ZoneAdditionData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act13SideConstData {
    pub prestige_desc_list: Vec<String>,
    pub daily_random_count: Option<i32>,
    pub daily_weight_initial: i32,
    pub daily_weight_complete: i32,
    pub agenda_recover: i32,
    pub agenda_max: i32,
    pub agenda_hint: i32,
    pub mission_pool_max: i32,
    pub mission_board_max: i32,
    pub item_random_list: Vec<ItemRandom>,
    pub unlock_prestige_cond: String,
    pub hot_spot_show_flag: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemRandom {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrgData {
    pub org_id: String,
    pub org_name: String,
    pub org_en_name: String,
    pub open_time: u64,
    pub principal_id_list: Vec<String>,
    pub prestige_list: Vec<Prestige>,
    pub agenda_count2_prestige_item_map: HashMap<String, PrestigeItem>,
    pub org_section_list: Vec<OrgSection>,
    pub principal_data_map: HashMap<String, PrincipalData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prestige {
    pub rank: String,
    pub threshold: i32,
    pub reward: Option<Reward>,
    pub news_count: i32,
    pub archive_count: i32,
    pub avg_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrestigeItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrgSection {
    pub section_name: String,
    pub sort_id: u16,
    pub group_data: GroupData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupData {
    pub group_id: String,
    pub group_name: String,
    pub org_id: String,
    pub mission_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalData {
    pub principal_id: String,
    pub principal_name: String,
    pub principal_en_name: String,
    pub avg_char_id: String,
    pub principal_desc_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongTermMissionData {
    pub mission_name: String,
    pub group_id: String,
    pub principal_id: String,
    pub finished_desc: String,
    pub section_sort_id: u16,
    pub have_stage_btn: bool,
    pub jump_stage_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyMissionData {
    pub id: String,
    pub sort_id: u16,
    pub description: String,
    pub mission_name: String,
    pub template: String,
    pub template_type: String,
    pub param: Vec<String>,
    pub rewards: Option<Vec<Reward>>,
    pub org_pool: Option<Vec<String>>,
    pub reward_pool: Option<Vec<String>>,
    pub jump_stage_id: String,
    pub agenda_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyRewardGroupData {
    pub group_id: String,
    pub rewards: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act13SideActivityReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveItemUnlockData {
    pub item_id: String,
    pub item_type: String,
    pub unlock_condition: String,
    pub param1: Option<String>,
    pub param2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HiddenAreaData {
    pub name: String,
    pub desc: String,
    pub preposed_stage: Vec<PreposedStage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreposedStage {
    pub stage_id: String,
    pub unlock_rank: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act13ZoneAdditionData {
    pub unlock_text: String,
    pub zone_class: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct17SideActivities {
    pub type_act17side: HashMap<String, Act17SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivity {
    pub place_data_map: HashMap<String, PlaceData>,
    pub event_data_map: HashMap<String, EventData>,
    pub landmark_node_data_map: HashMap<String, LandmarkNodeData>,
    pub story_node_data_map: HashMap<String, StoryNodeData>,
    pub battle_node_data_map: HashMap<String, BattleNodeData>,
    pub treasure_node_data_map: HashMap<String, TreasureNodeData>,
    pub event_node_data_map: HashMap<String, EventNodeData>,
    pub tech_node_data_map: HashMap<String, TechNodeData>,
    pub choice_node_data_map: HashMap<String, ChoiceNodeData>,
    pub archive_item_unlock_data_map: HashMap<String, Act17SideActivityArchiveItemUnlockData>,
    pub tech_tree_data_map: HashMap<String, TechTreeData>,
    pub tech_tree_branch_data_map: HashMap<String, TechTreeBranchData>,
    pub mainline_chapter_data_map: HashMap<String, MainlineChapterData>,
    pub mainline_data_map: HashMap<String, MainlineData>,
    pub zone_data_list: Vec<ZoneData>,
    pub const_data: Act17SideConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceData {
    pub place_id: String,
    pub place_desc: String,
    pub lock_event_id: Option<String>,
    pub zone_id: String,
    pub visible_cond_type: Option<String>,
    pub visible_params: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    pub event_id: String,
    pub event_pic: Option<String>,
    pub event_special_pic: Option<String>,
    pub event_title: String,
    pub event_des_list: Vec<String>,
    pub event_desc: String,
    pub event_type: String,
    pub event_param: Option<String>,
    pub reward_id: Option<String>,
    pub reward_type: Option<String>,
    pub place_id: String,
    pub point_id: String,
    pub zone_id: String,
    pub visible_cond_type: Option<String>,
    pub visible_params: Option<Vec<String>>,
    pub lock_type: Option<String>,
    pub lock_param: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LandmarkNodeData {
    pub node_id: String,
    pub landmark_id: String,
    pub landmark_name: String,
    pub landmark_pic: Option<String>,
    pub landmark_special_pic: Option<String>,
    pub landmark_des_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StoryNodeData {
    pub node_id: String,
    pub story_id: String,
    pub story_key: String,
    pub story_name: String,
    pub story_pic: String,
    pub confirm_des: String,
    pub story_des_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BattleNodeData {
    pub node_id: String,
    pub stage_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TreasureNodeData {
    pub node_id: String,
    pub treasure_id: String,
    pub treasure_name: String,
    pub treasure_pic: String,
    pub treasure_special_pic: Option<String>,
    pub end_event_id: String,
    pub confirm_des: String,
    pub treasure_des_list: Vec<String>,
    pub mission_id_list: Vec<String>,
    pub reward_list: Vec<Reward>,
    pub treasure_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivityReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventNodeData {
    pub node_id: String,
    pub event_id: String,
    pub end_event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TechNodeData {
    pub node_id: String,
    pub tech_tree_id: String,
    pub tech_tree_name: String,
    pub tech_pic: Option<String>,
    pub tech_special_pic: Option<String>,
    pub end_event_id: String,
    pub confirm_des: String,
    pub tech_des_list: Vec<String>,
    pub mission_id_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceNodeData {
    pub node_id: String,
    pub choice_pic: Option<String>,
    pub is_disposable: bool,
    pub choice_special_pic: Option<String>,
    pub choice_name: String,
    pub choice_des_list: Vec<String>,
    pub cancel_des: String,
    pub choice_num: i32,
    pub option_list: Vec<ChoiceOption>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceOption {
    pub can_repeat: bool,
    pub event_id: String,
    pub des: String,
    pub unlock_des: Option<String>,
    pub unlock_cond_type: Option<String>,
    pub unlock_params: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivityArchiveItemUnlockData {
    pub item_id: String,
    pub item_type: String,
    pub unlock_condition: String,
    pub node_id: Option<String>,
    pub stage_param: String,
    pub chapter_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TechTreeData {
    pub tech_tree_id: String,
    pub sort_id: u16,
    pub tech_tree_name: String,
    pub default_branch_id: String,
    pub lock_des: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TechTreeBranchData {
    pub tech_tree_branch_id: String,
    pub tech_tree_id: String,
    pub tech_tree_branch_name: String,
    pub tech_tree_branch_icon: String,
    pub tech_tree_branch_desc: String,
    pub rune_data: Act17SideActivityRuneData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivityRuneData {
    pub key: String,
    pub selector: Act17SideActivityRuneSelector,
    pub blackboard: Vec<Act17SideActivityBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivityRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideActivityBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainlineChapterData {
    pub chapter_id: String,
    pub chapter_des: String,
    pub chapter_icon: String,
    pub unlock_des: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainlineData {
    pub mainline_id: String,
    pub node_id: Option<String>,
    pub sort_id: u16,
    pub mission_sort: String,
    pub zone_id: String,
    pub mainline_des: String,
    pub focus_node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneData {
    pub zone_id: String,
    pub unlock_place_id: Option<String>,
    pub unlock_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act17SideConstData {
    pub tech_tree_unlock_event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct20SideActivities {
    pub type_act20side: HashMap<String, Act20SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act20SideActivity {
    pub zone_addition_data_map: HashMap<String, String>,
    pub resident_cart_datas: HashMap<String, ResidentCartData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResidentCartData {
    pub resident_pic: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct21SideActivities {
    pub type_act21side: HashMap<String, Act21SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act21SideActivity {
    pub zone_addition_data_map: HashMap<String, ZoneAdditionData>,
    pub const_data: Act21SideConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneAdditionData {
    pub zone_id: String,
    pub unlock_text: String,
    pub stage_unlock_text: Option<String>,
    pub entry_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act21SideConstData {
    pub line_connect_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LoginOnlyActivities {
    pub login_only: HashMap<String, LoginOnlyActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginOnlyActivity {
    pub description: String,
    pub item_list: Vec<LoginRewardItem>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SwitchOnlyActivities {
    pub switch_only: HashMap<String, SwitchOnlyActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchOnlyActivity {
    pub const_data: SwitchOnlyConstData,
    pub rewards: HashMap<String, Vec<SwitchRewardItem>>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
    pub rewards_title: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchOnlyConstData {
    pub activity_time: String,
    pub activity_rule: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MinistoryActivities {
    pub ministory: HashMap<String, MinistoryActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinistoryActivity {
    pub token_item_id: String,
    pub zone_desc_list: HashMap<String, ZoneDesc>,
    pub favor_up_list: HashMap<String, Value>,
    pub extra_drop_zone_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinistoryActivityZoneDesc {
    pub zone_id: String,
    pub unlock_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityTypes {
    pub roguelike: HashMap<String, Value>,
    pub multiplay_verify2: HashMap<String, MultiplayVerify2Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiplayVerify2Activity {
    pub select_step_data_list: Vec<SelectStepData>,
    pub identity_data_list: Vec<IdentityData>,
    pub map_type_datas: HashMap<String, MapTypeData>,
    pub map_datas: HashMap<String, MapData>,
    pub target_mission_datas: HashMap<String, TargetMissionData>,
    pub mile_stone_list: Vec<MileStone>,
    pub stage_star_reward_datas: HashMap<String, StageStarRewardData>,
    pub emoji_chat_datas: HashMap<String, EmojiChatData>,
    pub comment_datas: HashMap<String, CommentData>,
    pub tips_data_list: Vec<TipData>,
    pub report_data_list: Vec<ReportData>,
    pub temp_char_data_list: Vec<TempCharData>,
    pub multiplay_verify2_const_data: MultiplayVerify2ConstData,
    pub const_toast_data: ConstToastData,
    pub map_type_name_data_list: Vec<MapTypeNameData>,
    pub difficulty_name_data_list: Vec<DifficultyNameData>,
    pub buff_icon_datas: HashMap<String, BuffIconData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectStepData {
    pub step_type: String,
    pub sort_id: u16,
    pub time: i32,
    pub hint_time: i32,
    pub title: String,
    pub desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentityData {
    pub id: String,
    pub sort_id: u16,
    pub pic_id: String,
    #[serde(rename = "type")]
    pub identity_type: String,
    pub max_num: i32,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapTypeData {
    #[serde(rename = "type")]
    pub map_type: String,
    pub difficulty: String,
    pub squad_max: i32,
    pub match_unlock_mode_id: Option<String>,
    pub match_unlock_param: i32,
    pub stage_id_in_mode_list: Vec<String>,
    pub mode_icon_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    pub stage_id: String,
    pub mode_id: String,
    pub sort_id: u16,
    pub mission_id_list: Vec<String>,
    pub stage_small_preview_id: String,
    pub stage_big_preview_id: String,
    pub display_enemy_id_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetMissionData {
    pub id: String,
    pub sort_id: u16,
    pub title: String,
    pub battle_desc: String,
    pub description: String,
    pub star_num: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MileStone {
    pub mile_stone_id: String,
    pub mile_stone_lvl: i32,
    pub need_point_cnt: i32,
    pub reward_item: MultiplayVerify2ActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiplayVerify2ActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageStarRewardData {
    pub star_reward_datas: Vec<StarRewardData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarRewardData {
    pub star_num: i32,
    pub rewards: Vec<MultiplayVerify2ActivityRewardItem>,
    pub daily_mission_point: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmojiChatData {
    pub id: String,
    #[serde(rename = "type")]
    pub emoji_type: String,
    pub sort_id: u16,
    pub pic_id: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    pub id: String,
    #[serde(rename = "type")]
    pub comment_type: String,
    pub priority_id: i32,
    pub pic_id: String,
    pub txt: String,
    pub template: String,
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TipData {
    pub id: String,
    pub txt: String,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportData {
    pub id: String,
    pub sort_id: u16,
    pub txt: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TempCharData {
    pub char_id: String,
    pub level: i32,
    pub evolve_phase: String,
    pub main_skill_level: i32,
    pub specialize_level: i32,
    pub potential_rank: i32,
    pub favor_point: i32,
    pub skin_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiplayVerify2ConstData {
    pub milestone_id: String,
    pub max_unlock_num: i32,
    pub room_num_copy_desc: String,
    pub no_map_room_num_copy_desc: String,
    pub random_map_room_num_copy_desc: String,
    pub target_cd: i32,
    pub squad_min_num: i32,
    pub squad_max_num: i32,
    pub defense_tra_max: i32,
    pub defense_ord_max: i32,
    pub defense_dif_max: i32,
    pub stage_choose_anim_random_stage_id_list: Vec<String>,
    pub map_unlock_desc1: String,
    pub map_unlock_desc2: String,
    pub map_unlock_desc3: String,
    pub map_unlock_desc4: String,
    pub map_unlock_desc5: String,
    pub map_unlock_desc6: String,
    pub map_unlock_desc7: String,
    pub dif_unlock_cond: i32,
    pub ord_reward_stage_id: String,
    pub dif_reward_stage_id: String,
    pub max_match_time: i32,
    pub tips_switch_time: f32,
    pub ping_conds: Vec<PingCond>,
    pub chat_cd: i32,
    pub chat_time: i32,
    pub mark_cd: i32,
    pub mark_cond1: i32,
    pub mark_cond2: i32,
    pub daily_mission_param: i32,
    pub daily_mission_name: String,
    pub daily_mission_desc: String,
    pub daily_mission_rule: String,
    pub mission_desc: String,
    pub daily_mission_reward_item: RewardItem,
    pub normal_great_voice_star: i32,
    pub football_great_voice_num: i32,
    pub defence_great_voice_wave: i32,
    pub report_max_num: i32,
    pub report_text: String,
    pub reward_card_id: String,
    pub reward_card_text: String,
    pub reward_skin_id: String,
    pub reward_skin_text: String,
    pub max_retry_time_in_team_room: i32,
    pub max_retry_time_in_match_room: i32,
    pub max_retry_time_in_battle: i32,
    pub max_operator_delay: f32,
    pub max_play_speed: i32,
    pub delay_time_need_tip: i32,
    pub settle_retry_time: i32,
    pub mode_normal_unlock_mode_id: String,
    pub mode_normal_unlock_param: i32,
    pub mode_defence_unlock_mode_id: String,
    pub mode_defence_unlock_param: i32,
    pub mode_football_unlock_mode_id: String,
    pub mode_football_unlock_param: i32,
    pub tutorial_entry_story_id: String,
    pub tutorial_squad_story_id: String,
    pub team_unlock_stage_id: String,
    pub team_unlock_param: i32,
    pub train_partner_char_id: String,
    pub train_partner_char_skin_id: String,
    pub train_partner_player_name: String,
    pub train_partner_player_level: i32,
    pub train_partner_buff_id: String,
    pub train_partner_avatar_group_type: String,
    pub train_partner_avatar_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PingCond {
    pub cond: i32,
    pub txt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstToastData {
    pub no_room: String,
    pub full_room: String,
    pub room_id_format_error: String,
    pub room_id_copy_success: String,
    pub banned: String,
    pub server_overload: String,
    pub match_alive_failed: String,
    pub create_room_alive_failed: String,
    pub join_room_alive_failed: String,
    pub room_owner_revise_target: String,
    pub room_collaborator_revise_target: String,
    pub room_owner_revise_map: String,
    pub room_collaborator_revise_map: String,
    pub room_collaborator_join_room: String,
    pub room_collaborator_exit_room: String,
    pub continuous_clicks: String,
    pub match_no_project: String,
    pub report_no_project: String,
    pub other_mode_training_lock: String,
    pub team_lock: String,
    pub mentor_lock_tips: String,
    pub unlock_new_mode_in_match: String,
    pub unlock_new_stage_in_team: String,
    pub unlock_mentor_in_match: String,
    pub team_full_low: String,
    pub team_full_high: String,
    pub difficult_unlock: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapTypeNameData {
    pub map_type: String,
    pub type_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyNameData {
    pub difficulty: String,
    pub difficulty_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuffIconData {
    pub buff_id: String,
    pub icon_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InterlockActivities {
    pub interlock: HashMap<String, InterlockActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InterlockActivity {
    pub stage_addition_info_map: HashMap<String, StageAdditionInfo>,
    pub treasure_monster_map: HashMap<String, TreasureMonster>,
    pub special_assist_data: SpecialAssistData,
    pub mile_stone_item_list: Vec<InterlockActivityMileStoneItem>,
    pub final_stage_progress_map: HashMap<String, Vec<FinalStageProgress>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageAdditionInfo {
    pub stage_id: String,
    pub stage_type: String,
    pub lock_stage_key: Option<String>,
    pub lock_sort_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TreasureMonster {
    pub lock_stage_key: String,
    pub enemy_id: String,
    pub enemy_name: String,
    pub enemy_icon: String,
    pub enemy_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecialAssistData {
    pub char_id: String,
    pub potential_rank: i32,
    pub skill_index: i32,
    pub skin_id: String,
    pub skills: Vec<Skill>,
    pub current_equip: Option<String>,
    pub equip: Option<String>,
    pub main_skill_lvl: i32,
    pub evolve_phase: i32,
    pub level: i32,
    pub favor_point: i32,
    pub crisis_record: HashMap<String, String>,
    pub crisis_v2_record: HashMap<String, String>,
    pub current_tmpl: Option<String>,
    pub tmpl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub skill_id: String,
    pub specialize_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InterlockActivityMileStoneItem {
    pub mile_stone_id: String,
    pub order_id: i32,
    pub token_num: i32,
    pub item: InterlockActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InterlockActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FinalStageProgress {
    pub stage_id: String,
    pub kill_cnt: i32,
    pub ap_cost: i32,
    pub favor: i32,
    pub exp: i32,
    pub gold: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BossRushActivities {
    pub boss_rush: HashMap<String, BossRushActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivity {
    pub zone_addition_data_map: HashMap<String, BossRushActivityZoneAdditionData>,
    pub stage_group_map: HashMap<String, StageGroup>,
    pub stage_addition_data_map: HashMap<String, StageAdditionData>,
    pub stage_drop_data_map: HashMap<String, HashMap<String, BossRushActivityStageDrop>>,
    pub mission_addition_data_map: HashMap<String, MissionAdditionData>,
    pub team_data_map: HashMap<String, TeamData>,
    pub relic_list: Vec<Relic>,
    pub relic_level_info_data_map: HashMap<String, RelicLevelInfoData>,
    pub mile_stone_list: Vec<BossRushActivityMileStone>,
    pub best_wave_rune_list: Vec<BossRushActivity2RuneData>,
    pub const_data: BossRushConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityZoneAdditionData {
    pub unlock_text: String,
    pub display_start_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageGroup {
    pub stage_group_id: String,
    pub sort_id: u16,
    pub stage_group_name: String,
    pub stage_id_map: HashMap<String, String>,
    pub wave_boss_info: Vec<Vec<String>>,
    pub normal_stage_count: i32,
    pub is_hard_stage_group: bool,
    pub unlock_condition: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageAdditionData {
    pub stage_id: String,
    pub stage_type: String,
    pub stage_group_id: String,
    pub team_id_list: Vec<String>,
    pub unlock_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityStageDrop {
    pub clear_wave_count: i32,
    pub display_detail_rewards: Vec<DisplayDetailReward>,
    pub first_pass_rewards: Vec<String>,
    pub pass_rewards: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisplayDetailReward {
    pub occ_percent: String,
    pub drop_count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub drop_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionAdditionData {
    pub mission_id: String,
    pub is_relic_task: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamData {
    pub team_id: String,
    pub team_name: String,
    pub char_id_list: Vec<String>,
    pub team_buff_name: String,
    pub team_buff_des: String,
    pub team_buff_id: String,
    pub max_char_num: i32,
    pub rune_data: BossRushActivityRuneData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityRuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityRune {
    pub key: String,
    pub selector: BossRushActivityRuneSelector,
    pub blackboard: Vec<BossRushActivityBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub relic_id: String,
    pub sort_id: u16,
    pub name: String,
    pub icon: String,
    pub relic_task_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicLevelInfoData {
    pub relic_id: String,
    pub level_infos: HashMap<String, RelicLevelInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicLevelInfo {
    pub level: i32,
    pub effect_desc: String,
    pub rune_data: RelicRuneData,
    pub need_item_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicRuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<RelicRune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicRune {
    pub key: String,
    pub selector: RelicRuneSelector,
    pub blackboard: Vec<RelicBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityMileStone {
    pub mile_stone_id: String,
    pub mile_stone_lvl: i32,
    pub need_point_cnt: i32,
    pub reward_item: BossRushActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivity2RuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: Option<String>,
    pub runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivity2Rune {
    pub key: String,
    pub selector: BossRushActivity2RuneSelector,
    pub blackboard: Vec<BossRushActivity2BlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivity2RuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushActivity2BlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BossRushConstData {
    pub max_provided_char_num: i32,
    pub text_milestone_item_level_desc: String,
    pub milestone_point_id: String,
    pub relic_upgrade_item_id: String,
    pub default_relict_list: Vec<String>,
    pub reward_skin_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FloatParadeActivities {
    pub float_parade: HashMap<String, FloatParadeActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FloatParadeActivity {
    pub const_data: FloatParadeConstData,
    pub daily_data_dic: Vec<DailyData>,
    pub reward_pools: HashMap<String, HashMap<String, RewardPool>>,
    pub tactic_list: Vec<Tactic>,
    pub group_infos: HashMap<String, GroupInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FloatParadeConstData {
    pub city_name: String,
    pub city_name_pic: String,
    pub low_standard: f64,
    pub variation_title: String,
    pub rule_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyData {
    pub day_index: i32,
    pub date_name: String,
    pub place_name: String,
    pub place_en_name: String,
    pub place_pic: String,
    pub event_group_id: String,
    pub ext_reward: Option<ExtReward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtReward {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardPool {
    pub grp_id: String,
    pub id: String,
    #[serde(rename = "type")]
    pub reward_type: String,
    pub name: String,
    pub desc: Option<String>,
    pub reward: FloatParadeActivityReward,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FloatParadeActivityReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tactic {
    pub id: i32,
    pub name: String,
    pub pack_name: String,
    pub brief_name: String,
    pub reward_var: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub group_id: String,
    pub name: String,
    pub start_day: i32,
    pub end_day: i32,
    pub ext_reward_day: i32,
    pub ext_reward_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MainBuffActivities {
    pub main_buff: HashMap<String, MainBuffActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainBuffActivity {
    pub mission_group_list: HashMap<String, MainBuffActivityMissionGroup>,
    pub period_data_list: Vec<MainBuffActivityPeriodData>,
    pub ap_supply_out_of_date_dict: HashMap<String, u64>,
    pub const_data: MainBuffConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainBuffActivityMissionGroup {
    pub id: String,
    pub bind_banner: String,
    pub sort_id: u16,
    pub zone_id: String,
    pub mission_id_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainBuffActivityPeriodData {
    pub id: String,
    pub start_time: u64,
    pub end_time: u64,
    pub favor_up_char_desc: String,
    pub favor_up_img_name: String,
    pub new_chapter_img_name: String,
    pub new_chapter_zone_id: Option<String>,
    pub step_data_list: Vec<StepData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StepData {
    pub is_block: bool,
    pub favor_up_desc: Option<String>,
    pub unlock_desc: Option<String>,
    pub bind_stage_id: Option<String>,
    pub block_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainBuffConstData {
    pub favor_up_stage_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct24SideActivities {
    pub type_act24side: HashMap<String, Act24SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivity {
    pub tool_data_list: HashMap<String, ToolData>,
    pub meal_data_list: HashMap<String, MealData>,
    pub melding_dict: HashMap<String, MeldingData>,
    pub melding_gacha_box_data_list: HashMap<String, MeldingGachaBoxData>,
    pub melding_gacha_box_good_data_map: HashMap<String, Vec<MeldingGachaBoxGoodData>>,
    pub meal_welcome_txt_data_map: HashMap<String, String>,
    pub zone_addition_data_map: HashMap<String, Act24SideActivityZoneAdditionData>,
    pub quest_stage_list: Vec<QuestStage>,
    pub mission_data_list: HashMap<String, Act24SideActivityMissionData>,
    pub melding_drop_dict: HashMap<String, MeldingDropData>,
    pub stage_map_preview_dict: HashMap<String, Vec<String>>,
    pub const_data: Act24SideConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolData {
    pub tool_id: String,
    pub sort_id: u16,
    pub tool_name: String,
    pub tool_desc: String,
    pub tool_icon1: String,
    pub tool_icon2: String,
    pub tool_unlock_desc: String,
    pub tool_buff_id: String,
    pub rune_data: RuneData,
    pub tool_stage_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityRuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<Act24SideActivityRune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityRune {
    pub key: String,
    pub selector: Act24SideActivityRuneSelector,
    pub blackboard: Vec<Act24SideActivityBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MealData {
    pub meal_id: String,
    pub sort_id: u16,
    pub meal_name: String,
    pub meal_effect_desc: String,
    pub meal_desc: String,
    pub meal_icon: String,
    pub meal_cost: i32,
    pub meal_reward_ap: i32,
    pub meal_reward_item_info: Act24SideActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeldingData {
    pub melding_id: String,
    pub sort_id: u16,
    pub melding_price: i32,
    pub rarity: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeldingGachaBoxData {
    pub gacha_box_id: String,
    pub gacha_sort_id: u16,
    pub gacha_icon: String,
    pub gacha_box_name: String,
    pub gacha_cost: i32,
    pub gacha_times_limit: i32,
    pub theme_color: String,
    pub remain_item_bg_color: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeldingGachaBoxGoodData {
    pub good_id: String,
    pub gacha_box_id: String,
    pub order_id: i32,
    pub item_id: String,
    pub item_type: String,
    pub display_type: String,
    pub per_count: i32,
    pub total_count: i32,
    pub gacha_type: String,
    pub weight: i32,
    pub gacha_order_id: i32,
    pub gacha_num: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityZoneAdditionData {
    pub zone_id: String,
    pub zone_icon: String,
    pub unlock_text: String,
    pub display_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuestStage {
    pub stage_id: String,
    pub stage_rank: i32,
    pub sort_id: u16,
    pub is_urgent_stage: bool,
    pub is_dragon_stage: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityMissionData {
    pub task_type_name: String,
    pub task_type_icon: String,
    pub task_type: String,
    pub task_title: String,
    pub task_client: String,
    pub task_client_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeldingDropData {
    pub first_pass_rewards: Option<Vec<Reward>>,
    pub first_complete_rewards: Option<Vec<Reward>>,
    pub pass_rewards: Option<Vec<Reward>>,
    pub complete_rewards: Option<Vec<Reward>>,
    pub display_rewards: Vec<DisplayReward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideActivityReward {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisplayReward {
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub drop_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act24SideConstData {
    pub stage_unlock_tool_desc: String,
    pub meal_lack_money: String,
    pub meal_day_times_limit: i32,
    pub tool_maximum: i32,
    pub stage_can_not_use_to_tool: Vec<String>,
    pub gacha_default_prob: f64,
    pub gacha_extra_prob: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct25SideActivities {
    pub type_act25side: HashMap<String, Act25SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivity {
    pub token_item_id: String,
    pub const_data: Act25SideConstData,
    pub zone_desc_list: HashMap<String, Act25SideActivityZoneDesc>,
    pub archive_item_data: HashMap<String, ArchiveItemData>,
    pub arc_map_info_data: HashMap<String, ArcMapInfoData>,
    pub area_info_data: HashMap<String, AreaInfoData>,
    pub area_mission_data: HashMap<String, AreaMissionData>,
    pub battle_performance_data: HashMap<String, BattlePerformanceData>,
    pub key_data: HashMap<String, KeyData>,
    pub fog_unlock_data: HashMap<String, FogUnlockData>,
    pub farm_list: Vec<FarmData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideConstData {
    pub get_daily_count: i32,
    pub cost_name: String,
    pub cost_desc: String,
    pub cost_limit: i32,
    pub reward_limit: i32,
    pub research_unlock_text: String,
    pub harvest_reward: HarvestReward,
    pub cost_count: i32,
    pub cost_count_limit: i32,
    pub basic_progress: i32,
    pub harvest_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HarvestReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityZoneDesc {
    pub zone_id: String,
    pub unlock_text: String,
    pub display_start_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveItemData {
    pub item_id: String,
    pub item_type: String,
    pub item_unlock_type: String,
    pub item_unlock_param: String,
    pub unlock_desc: Option<String>,
    pub icon_id: Option<String>,
    pub item_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArcMapInfoData {
    pub object_id: String,
    #[serde(rename = "type")]
    pub info_type: String,
    pub number_id: String,
    pub area_id: String,
    pub sort_id: u16,
    pub position: i32,
    pub has_dot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AreaInfoData {
    pub area_id: String,
    pub sort_id: u16,
    pub area_icon: String,
    pub area_name: String,
    pub unlock_text: String,
    pub preposed_stage: String,
    pub area_initial_desc: String,
    pub area_ending_desc: String,
    pub area_ending_aud: String,
    pub reward: Reward,
    pub final_id: String,
    pub area_new_icon: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AreaMissionData {
    pub id: String,
    pub area_id: String,
    pub preposed_mission_id: Option<String>,
    pub sort_id: u16,
    pub is_zone: bool,
    pub stage_id: String,
    pub cost_count: i32,
    pub transform: i32,
    pub progress: i32,
    pub progress_pic_id: String,
    pub template: Option<String>,
    pub template_type: i32,
    pub desc: String,
    pub param: Option<String>,
    pub rewards: Vec<Act25SideActivityReward>,
    pub archive_items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BattlePerformanceData {
    pub item_id: String,
    pub sort_id: u16,
    pub item_name: String,
    pub item_icon: String,
    pub item_desc: String,
    pub item_tech_type: String,
    pub rune_data: Act25SideActivityRuneData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityRuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<Act25SideActivityRune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityRune {
    pub key: String,
    pub selector: Act25SideActivityRuneSelector,
    pub blackboard: Vec<Act25SideActivityBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act25SideActivityBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    pub key_id: String,
    pub key_name: String,
    pub key_icon: String,
    pub toast_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FogUnlockData {
    pub lock_id: String,
    pub locked_collection_icon_id: String,
    pub unlocked_collection_icon_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FarmData {
    pub transform: i32,
    pub unit_time: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct27SideActivities {
    pub type_act27side: HashMap<String, Act27SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act27SideActivity {
    pub good_data_map: HashMap<String, GoodData>,
    pub mile_stone_list: Vec<Act27SideActivityMileStone>,
    pub good_launch_data_list: Vec<GoodLaunchData>,
    pub shop_data_map: HashMap<String, ShopData>,
    pub inquire_data_list: Vec<InquireData>,
    pub dyn_entry_switch_data: Vec<DynEntrySwitchData>,
    pub zone_addition_data_map: HashMap<String, Act27SideActivityZoneAdditionData>,
    pub const_data: Act27SideConstData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoodData {
    pub id: String,
    pub name: String,
    pub type_desc: String,
    pub icon_id: String,
    pub launch_icon_id: String,
    pub purchase_price: Vec<i32>,
    pub selling_price_list: Vec<i32>,
    pub sell_shop_list: Vec<String>,
    pub is_permanent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoodLaunchData {
    pub group_id: String,
    pub start_time: u64,
    pub stage_id: Option<String>,
    pub code: Option<String>,
    pub drink_id: String,
    pub food_id: String,
    pub souvenir_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act27SideActivityMileStone {
    pub mile_stone_id: String,
    pub mile_stone_lvl: i32,
    pub need_point_cnt: i32,
    pub reward_item: Act27SideActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act27SideActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShopData {
    pub shop_id: String,
    pub sort_id: u16,
    pub name: String,
    pub icon_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InquireData {
    pub mile_stone_pt: i32,
    pub inquire_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynEntrySwitchData {
    pub entry_id: String,
    pub start_hour: i32,
    pub signal_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act27SideActivityZoneAdditionData {
    pub zone_id: String,
    pub unlock_text: String,
    pub display_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act27SideConstData {
    pub stage_id: String,
    pub stage_code: String,
    pub purchase_price_name: Vec<String>,
    pub furni_reward_list: Vec<FurniReward>,
    pub prize_text: String,
    pub player_shop_id: String,
    pub milestone_point_name: String,
    pub inquire_panel_title: String,
    pub inquire_panel_desc: String,
    pub gain123: Vec<f64>,
    pub gain113: Vec<f64>,
    pub gain122: Vec<f64>,
    pub gain111: Vec<f64>,
    pub gain11_none: Vec<f64>,
    pub gain12_none: Vec<f64>,
    pub campaign_enemy_cnt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FurniReward {
    pub furni_id: String,
    pub point_num: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct42D0Activities {
    pub type_act42d0: HashMap<String, Act42D0Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0Activity {
    pub area_info_data: HashMap<String, AreaInfo>,
    pub stage_info_data: HashMap<String, Act42D0SideActivityStageInfo>,
    pub effect_group_info_data: HashMap<String, EffectGroupInfo>,
    pub effect_info_data: HashMap<String, EffectInfo>,
    pub challenge_info_data: HashMap<String, ChallengeInfo>,
    pub stage_rating_info_data: HashMap<String, StageRatingInfo>,
    pub milestone_data: Vec<Act42D0Activity2MilestoneData>,
    pub const_data: Act42D0ConstData,
    pub track_point_period_data: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AreaInfo {
    pub area_id: String,
    pub sort_id: u16,
    pub area_code: String,
    pub area_name: String,
    pub difficulty: String,
    pub area_desc: String,
    pub cost_limit: i32,
    pub boss_icon: String,
    pub boss_id: Option<String>,
    pub next_area_stage: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0SideActivityStageInfo {
    pub stage_id: String,
    pub area_id: String,
    pub stage_code: String,
    pub sort_id: u16,
    pub stage_desc: Vec<String>,
    pub level_id: String,
    pub code: String,
    pub name: String,
    pub loading_pic_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectGroupInfo {
    pub effect_group_id: String,
    pub sort_id: u16,
    pub effect_group_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectInfo {
    pub effect_id: String,
    pub effect_group_id: String,
    pub row: i32,
    pub col: i32,
    pub effect_name: String,
    pub effect_icon: String,
    pub cost: i32,
    pub effect_desc: String,
    pub unlock_time: u64,
    pub rune_data: Act42D0ActivityRuneData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityRuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<Act42D0ActivityRune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityRune {
    pub key: String,
    pub selector: Act42D0ActivityRuneSelector,
    pub blackboard: Vec<Act42D0ActivityBlackboardItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityRuneSelector {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityBlackboardItem {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeInfo {
    pub stage_id: String,
    pub stage_desc: String,
    pub start_ts: u64,
    pub end_ts: u64,
    pub level_id: String,
    pub code: String,
    pub name: String,
    pub loading_pic_id: String,
    pub challenge_mission_data: Vec<ChallengeMission>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeMission {
    pub mission_id: String,
    pub sort_id: u16,
    pub stage_id: String,
    pub mission_desc: String,
    pub milestone_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageRatingInfo {
    pub stage_id: String,
    pub area_id: String,
    pub milestone_data: Vec<Act42D0ActivityMilestoneData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityMilestoneData {
    pub rating_level: i32,
    pub cost_up_limit: i32,
    pub achivement: String,
    pub icon: String,
    pub milestone_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0Activity2MilestoneData {
    pub milestone_id: String,
    pub order_id: i32,
    pub token_num: i32,
    pub item: Act42D0ActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act42D0ConstData {
    pub milestone_id: String,
    pub strife_name: String,
    pub strife_desc: String,
    pub unlock_desc: String,
    pub reward_desc: String,
    pub trauma_desc: String,
    pub milestone_area_name: String,
    pub trauma_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct29SideActivities {
    pub type_act29side: HashMap<String, Act29SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act29SideActivity {
    pub frag_data_map: HashMap<String, FragData>,
    pub orche_data_map: HashMap<String, OrcheData>,
    pub product_group_data_map: HashMap<String, ProductGroupData>,
    pub product_data_map: HashMap<String, ProductData>,
    pub form_data_map: HashMap<String, FormData>,
    pub invest_result_data_map: HashMap<String, InvestResultData>,
    pub invest_data_map: HashMap<String, InvestData>,
    pub major_invest_id_list: Vec<String>,
    pub rare_invest_id_list: Vec<String>,
    pub const_data: Act29SideConstData,
    pub zone_addition_data_map: HashMap<String, Act29SideActivityZoneAdditionData>,
    pub music_data_map: Vec<MusicData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragData {
    pub frag_id: String,
    pub sort_id: u16,
    pub frag_name: String,
    pub frag_icon: String,
    pub frag_store_icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrcheData {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub icon: String,
    pub sort_id: u16,
    pub orche_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductGroupData {
    pub group_id: String,
    pub group_name: String,
    pub group_icon: String,
    pub group_desc: String,
    pub default_bgm_signal: String,
    pub product_list: Vec<String>,
    pub group_eng_name: String,
    pub group_small_name: String,
    pub group_type_icon: String,
    pub group_store_icon_id: String,
    pub group_type_base_pic: String,
    pub group_type_eye_icon: String,
    pub group_sort_id: u16,
    pub form_list: Vec<String>,
    pub sheet_id: String,
    pub sheet_num: i32,
    pub sheet_rotate_spd: f64,
    pub product_type: String,
    pub product_desc_color: String,
    pub play_tint_color: String,
    pub confirm_tint_color: String,
    pub confirm_desc_color: String,
    pub bag_theme_color: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductData {
    pub id: String,
    pub orche_id: String,
    pub group_id: String,
    pub form_id: String,
    pub music_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormData {
    pub form_id: String,
    pub frag_id_list: Vec<String>,
    pub form_desc: String,
    pub product_id_dict: HashMap<String, String>,
    pub without_orche_product_id: String,
    pub group_id: String,
    pub form_sort_id: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvestResultData {
    pub result_id: String,
    pub result_title: String,
    pub result_desc1: String,
    pub result_desc2: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvestData {
    pub invest_id: String,
    pub invest_type: String,
    pub invest_npc_name: String,
    pub story_id: String,
    pub invest_npc_pic: String,
    pub invest_npc_avatar_pic: String,
    pub major_npc_pic: String,
    pub major_npc_black_pic: String,
    pub reward: Reward,
    pub invest_suc_result_id: String,
    pub invest_fail_result_id: String,
    pub invest_rare_result_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act29SideActivityReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act29SideConstData {
    pub major_invest_unlock_item_name: String,
    pub wrong_tips_trigger_time: i32,
    pub major_invest_complete_img_id: String,
    pub major_invest_unknown_avatar_id: String,
    pub major_invest_detail_desc1: String,
    pub major_invest_detail_desc2: String,
    pub major_invest_detail_desc3: String,
    pub major_invest_detail_desc4: String,
    pub hidden_invest_img_id: String,
    pub hidden_invest_head_img_id: String,
    pub hidden_invest_npc_name: String,
    pub unlock_level_id: String,
    pub invest_result_hint: String,
    pub invest_unlock_text: String,
    pub no_orche_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act29SideActivityZoneAdditionData {
    pub zone_id: String,
    pub unlock_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicData {
    pub group_id: String,
    pub orche_id: String,
    pub music_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Year5GeneralActivities {
    pub year_5_general: HashMap<String, Year5GeneralActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Year5GeneralActivity {
    pub const_data: Year5GeneralConstData,
    pub unlimited_ap_rewards: Vec<UnlimitedApReward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Year5GeneralConstData {
    pub rew_point: i32,
    pub rew_main_desc: String,
    pub rew_ap_desc: String,
    pub rew_end_desc: String,
    pub act_primary_desc: String,
    pub act_entry_desc: String,
    pub act_secondary_desc: String,
    pub act_reward_desc: String,
    pub mission_archive_topic_id: String,
    pub mission_archive_unlock_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlimitedApReward {
    pub reward_index: i32,
    pub reward_item: Year5GeneralActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Year5GeneralActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TypeAct35SideActivities {
    pub type_act35side: HashMap<String, Act35SideActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideActivity {
    pub challenge_data_map: HashMap<String, Act35SideActivityChallengeData>,
    pub card_data_map: HashMap<String, CardData>,
    pub material_data_map: HashMap<String, MaterialData>,
    pub dialogue_group_data_map: HashMap<String, DialogueGroupData>,
    pub const_data: Act35SideConstData,
    pub mile_stone_list: Vec<Act35SideActivityMileStone>,
    pub zone_addition_data_map: HashMap<String, Act35SideActivityZoneAdditionData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideActivityChallengeData {
    pub challenge_id: String,
    pub challenge_name: String,
    pub challenge_desc: String,
    pub sort_id: u16,
    pub challenge_pic_id: String,
    pub challenge_icon_id: String,
    pub open_time: u64,
    pub preposed_challenge_id: Option<String>,
    pub pass_round: i32,
    pub pass_round_score: i32,
    pub round_id_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub card_id: String,
    pub sort_id: u16,
    pub rank: i32,
    pub card_face: String,
    pub card_pic: String,
    pub level_data_list: Vec<CardLevelData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardLevelData {
    pub card_level: i32,
    pub card_name: String,
    pub card_desc: String,
    pub input_material_list: Vec<MaterialData>,
    pub output_material_list: Vec<MaterialData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaterialData {
    pub material_id: String,
    pub count: i32,
    pub material_icon: String,
    pub material_name: String,
    pub material_rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DialogueGroupData {
    #[serde(rename = "type")]
    pub dialogue_type: String,
    pub dialog_data_list: Vec<DialogData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DialogData {
    pub sort_id: u16,
    pub icon_id: String,
    pub name: String,
    pub content: String,
    pub bg_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideConstData {
    pub campaign_stage_id: String,
    pub campaign_enemy_cnt: i32,
    pub milestone_grand_reward_info_list: Vec<MilestoneGrandRewardInfo>,
    pub unlock_level_id: String,
    pub bird_spine_low_rate: f64,
    pub bird_spine_high_rate: f64,
    pub card_max_level: i32,
    pub max_slot_cnt: i32,
    pub card_refresh_num: i32,
    pub init_slot_cnt: i32,
    pub bonus_material_id: String,
    pub intro_round_id_list: Vec<String>,
    pub challenge_unlock_text: String,
    pub slot_unlock_text: String,
    pub estimate_ratio: i32,
    pub carving_unlock_toast_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneGrandRewardInfo {
    pub item_name: String,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideActivityMileStone {
    pub mile_stone_id: String,
    pub mile_stone_lvl: i32,
    pub need_point_cnt: i32,
    pub reward_item: Act35SideActivityRewardItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideActivityRewardItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Act35SideActivityZoneAdditionData {
    pub zone_id: String,
    pub unlock_text: String,
}

// End of Activity Structs.

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainlineBp {
    pub period_data_list: Vec<PeriodData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeriodData {
    pub period_id: String,
    pub start_ts: u64,
    pub end_ts: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItems {
    #[serde(flatten)]
    pub items: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PubBehavior {
    pub sort_id: u16,
    pub all_behavior_id: String,
    pub display_order: i32,
    pub all_behavior_desc: String,
    pub requiring_value: u64,
    pub require_repeat_completion: bool,
    pub reward_received_desc: String,
    pub rewards: Vec<PubBehaviorReward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PubBehaviorReward {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInAllPlayerDay {
    pub item_list: Vec<CheckInAllPlayerItem>,
    pub order: i32,
    pub key_item: bool,
    pub show_item_order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckInAllPlayerItem {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub comp_id: String,
    pub sort_id: u16,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncPoints {
    #[serde(flatten)]
    pub activities: HashMap<String, Vec<u64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynActs {
    #[serde(flatten)]
    pub activities: HashMap<String, PrayActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrayActivity {
    pub rule1: String,
    pub rule2: String,
    pub slot_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageRewardsData {
    pub stage_rewards_dict: HashMap<String, HashMap<String, Vec<StageReward>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageReward {
    pub occ_percent: String,
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub drop_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActTheme {
    pub id: String,
    #[serde(rename = "type")]
    pub theme_type: String,
    pub func_id: String,
    pub end_ts: u64,
    pub sort_id: u16,
    pub item_id: String,
    pub time_nodes: Vec<TimeNode>,
    pub start_ts: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeNode {
    pub title: String,
    pub ts: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActFunData {
    pub stages: HashMap<String, Stage>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockCondition {
    pub stage_id: String,
    pub complete_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StageDrop {
    pub id: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub drop_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CarData {
    pub car_dict: HashMap<String, Car>,
    pub rune_data_dict: HashMap<String, RuneData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuneData {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: String,
    pub description: String,
    pub runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub key: String,
    pub selector: Selector,
    pub blackboard: Vec<Blackboard>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blackboard {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartStages {
    pub cart_stages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstData {
    pub character_name: String,
    pub skin_name: String,
    pub car_item_unlock_stage_id: String,
    pub car_item_unlock_desc: String,
    pub sp_level_unlock_item_cnt: i32,
    pub mile_stone_base_interval: i32,
    pub sp_stage_ids: Vec<String>,
    pub car_frame_default_color: String,
    pub campaign_enemy_cnt: i32,
    pub campaign_stage_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SiracusaData {
    pub area_data_map: HashMap<String, AreaData>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KvSwitchData {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityKvSwitch>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityKvSwitch {
    pub kv_switch_info: HashMap<String, KvSwitchInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KvSwitchInfo {
    pub is_default: bool,
    pub display_time: u64,
    pub zone_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HiddenStage {
    pub stage_id: String,
    pub encoded_name: String,
    pub show_stage_id: String,
    pub reward_diamond: bool,
    pub missions: Vec<HiddenStageMission>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionArchives {
    #[serde(flatten)]
    pub archives: HashMap<String, MissionArchive>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionArchive {
    pub topic_id: String,
    pub zones: Vec<String>,
    pub nodes: Vec<ArchiveNode>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveNode {
    pub node_id: String,
    pub title: String,
    pub unlock_desc: String,
    pub clips: Vec<ArchiveClip>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveClip {
    pub char_id: String,
    pub voice_id: String,
    pub index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FifthAnnivExploreData {
    pub explore_group_data: HashMap<String, ExploreGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StringRes {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityStringRes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityStringRes {
    pub text_progress_format_can_claim: String,
    pub text_progress_format_cannot_claim: String,
    pub text_detail_color_can_claim: String,
    pub text_detail_color_cannot_claim: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTraps {
    #[serde(flatten)]
    pub activities: HashMap<String, ActivityTrapData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTrapData {
    pub template_traps: HashMap<String, TemplateTrap>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemplateTrap {
    pub trap_id: String,
    pub sort_id: u16,
    pub trap_name: String,
    pub trap_desc: String,
    pub trap_text: String,
    pub trap_task_id: String,
    pub trap_unlock_desc: String,
    pub trap_buff_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneDataDict {
    #[serde(flatten)]
    pub runes: HashMap<String, TrapRune>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrapRune {
    pub id: String,
    pub points: f64,
    pub mutex_group_key: Option<String>,
    pub description: String,
    pub runes: Vec<TrapRuneEffect>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneEffect {
    pub key: String,
    pub selector: TrapRuneSelector,
    pub blackboard: Vec<TrapRuneBlackboard>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrapRuneBlackboard {
    pub key: String,
    pub value: f64,
    pub value_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTemplateMissionStyles {
    #[serde(flatten)]
    pub styles: HashMap<String, TemplateMissionStyle>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    #[serde(rename = "type")]
    pub reward_type: String,
    pub id: String,
    pub count: i32,
}
