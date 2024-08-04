use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    complete_times: u16,
    has_battle_replay: u16,
    no_cost_cnt: u16,
    practice_times: u16,
    stage_id: String,
    start_times: u16,
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
enum CowLevelValue {
    Bool(bool),
    Arr([u8; 2]),
}

impl CowLevelValue {
    pub fn default() -> Self {
        Self::Bool(true)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CowLevel {
    id: String,
    #[serde(rename = "type")]
    t: String,
    val: [CowLevelValue; 1],
    fts: u64,
    rts: u64,
}

impl CowLevel {
    pub fn new(id: String) -> Self {
        Self {
            id,
            t: String::from("STAGE"),
            val: [CowLevelValue::default()],
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
