use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryReviewTable {
    #[serde(flatten)]
    pub story_review_table: HashMap<String, StoryReview>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryReview {
    pub info_unlock_datas: Vec<StoryReviewInfoUnlockData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryReviewInfoUnlockData {
    pub story_id: String,
}

impl LoadTable for StoryReviewTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/story_review_table.json")?)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryReviewMetaTable {
    pub mini_act_trial_data: StoryReviewMetaMiniActTrialData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryReviewMetaMiniActTrialData {
    pub mini_act_trial_data_map: HashMap<String, MiniActTrialData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniActTrialData {
    pub reward_list: Vec<MiniActTrialDataReward>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniActTrialDataReward {
    pub trial_reward_id: String,
}

impl LoadTable for StoryReviewMetaTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/story_review_meta_table.json")?)?)
    }
}
