use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dungeon {
    pub stages: HashMap<String, Stage>,
    pub cow_level: HashMap<String, CowLevel>,
    pub hide_stages: HashMap<String, HideStage>,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            stages: HashMap::new(),
            cow_level: HashMap::new(),
            hide_stages: HashMap::new(),
        }
    }

    pub fn set_stage(&mut self, stage_id: String) {
        self.stages.insert(stage_id.clone(), Stage::new(stage_id));
    }

    pub fn set_cow_level(&mut self, stage_id: String) {
        self.cow_level.insert(stage_id.clone(), CowLevel::new(stage_id));
    }

    pub fn set_hide_stage(&mut self, hide_id: String) {
        self.hide_stages.insert(hide_id, HideStage::default());
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub complete_times: u16,
    pub has_battle_replay: u16,
    pub no_cost_cnt: u16,
    pub practice_times: u16,
    pub stage_id: String,
    pub start_times: u16,
    pub state: u8,
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
#[serde(untagged)]
enum CowLevelValue {
    Bool(bool),
    Arr([u8; 2]),
}

impl Default for CowLevelValue {
    fn default() -> Self {
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

impl Default for HideStage {
    fn default() -> Self {
        Self {
            missions: [HideStageMission { value: 1, target: 1 }; 2],
            unlock: 1,
        }
    }
}
