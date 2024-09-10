use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::PlayerDataDelta;

pub mod detail;
pub mod keypoint;
pub mod stage;

use stage::CrisisStageData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crisis {
    info: CrisisInfo,
    ts: u64,
    player_data_delta: PlayerDataDelta,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrisisInfo {
    pub season_id: String,
    pub map_stage_data_map: HashMap<String, CrisisStageData>,
}
