use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2Table {
    pub season_info_data_map: HashMap<String, CrisisV2SeasonData>,
    pub score_level_to_appraise_data_map: HashMap<String, CrisisV2Appraisal>,
    pub const_data: CrisisV2ConstData,
}

impl CrisisV2Table {
    pub fn load() -> Self {
        from_reader(File::open("./data/excel/crisis_v2_table.json").unwrap()).unwrap()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2Appraisal {
    pub appraisal_type: CrisisV2AppraisalType,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CrisisV2AppraisalType {
    RankD,
    RankC,
    RankB,
    RankA,
    RankS,
    RankSS,
    RankSSS,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2ConstData {
    pub sys_start_time: u64,
    pub black_score_threshold: u8,
    pub red_score_threshold: u8,
    pub detail_bkg_red_threshold: u16,
    pub voice_grade: u16,
    pub season_button_unlock_info: u64,
    pub shop_coin_id: String,
    pub hard_bgm_switch_score: u16,
    pub stage_id: String,
    pub hide_todo_when_stage_finish: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisV2SeasonData {
    pub season_id: String,
    pub name: String,
    pub start_ts: u64,
    pub end_ts: u64,
    pub medal_group_id: String,
    pub medal_id: String,
    pub theme_color1: String,
    pub theme_color2: String,
    pub theme_color3: String,
    pub season_bgm: String,
    pub season_bgm_challenge: String,
    pub crisis_v2_season_code: String,
}
