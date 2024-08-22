use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::keypoint::CrisisInfoDetailDataChallengeNodeKeypoint;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailData {
    pub comment_data_map: HashMap<String, CrisisInfoDetailDataComment>,
    pub challenge_node_data_map: HashMap<String, CrisisInfoDetailDataChallengeNodeKeypoint>,
    pub group_desc_data_map: HashMap<String, CrisisInfoDetailGroupDescData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailDataComment {
    pub condition: String,
    pub desc: String,
    pub id: String,
    pub param_list: Vec<String>,
    pub sort_id: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfoDetailGroupDescData {
    pub desc: String,
    pub sort_id: u32,
}
