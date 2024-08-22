use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailDataChallengeNodeKeypoint {
    pub mission_type: String,
    pub mission_param_list: Vec<String>,
    pub slot_id: String,
    pub preview_title: String,
    pub preview_desc: String,
    pub mission_sort_id: u8,
    pub reward_list: Vec<CrisisInfoDetailDataChallengeNodeKeypointReward>,
    pub required_slot_count: u8,
    pub slot_id_list: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailDataChallengeNodeKeypointReward {
    pub reward: CrisisInfoDetailDataChallengeNodeKeypointRewardData,
    pub is_time_limit: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailDataChallengeNodeKeypointRewardData {
    pub id: String,
    pub count: u32,
    #[serde(rename = "type")]
    pub t: String,
}
