use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stage {
    #[serde(rename = "completeTimes")]
    complete_times: u16,
    #[serde(rename = "hasBattleReplay")]
    has_battle_replay: u16,
    #[serde(rename = "noCostCnt")]
    no_cost_cnt: u16,
    #[serde(rename = "practiceTimes")]
    practice_times: u16,
    #[serde(rename = "stageId")]
    stage_id: String,
    #[serde(rename = "startTimes")]
    start_times: u16,
    #[serde(rename = "state")]
    state: u8,
}

impl Stage {
    pub fn new(stage_id: String) -> Self {
        Self {
            complete_times: 1,
            has_battle_replay: 0,
            no_cost_cnt: 0,
            practice_times: 0,
            stage_id,
            start_times: 1,
            state: 3,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CowLevel<'a> {
    id: String,
    #[serde(rename = "type")]
    t: &'a str,
    val: [u8; 0],
    fts: u64,
    rts: u64,
}

impl<'a> CowLevel<'a> {
    pub fn new(id: String) -> Self {
        Self {
            id,
            t: "STAGE",
            val: [],
            fts: 1600000000,
            rts: 1600000000,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct HideStageMission {
    value: u8,
    target: u8,
}

#[derive(Serialize, Deserialize)]
pub struct HideStage {
    missions: [HideStageMission; 2],
    unlock: u8,
}

impl HideStage {
    pub const fn default() -> Self {
        Self {
            missions: [HideStageMission { value: 1, target: 1 }; 2],
            unlock: 1,
        }
    }
}
