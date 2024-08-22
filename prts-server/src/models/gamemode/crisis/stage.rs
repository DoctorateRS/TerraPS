use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisStageData {
    code: String,
    description: String,
    level_id: String,
    loading_pic_id: String,
    logo_pic_id: String,
    map_id: String,
    name: String,
    pic_id: String,
    reward_end_time: i64,
    stage_id: String,
    stage_type: String,
    start_time: i64,
}
