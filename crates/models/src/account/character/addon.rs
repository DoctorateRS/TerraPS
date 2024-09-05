use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common_utils::time;

#[derive(Deserialize, Serialize, Default)]
pub struct CharAddon {
    pub story: HashMap<String, CharStoryAddon>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub stage: HashMap<String, CharStageAddon>,
}

impl CharAddon {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_story(&mut self, id: String) {
        self.story.insert(id, CharStoryAddon::default());
    }

    pub fn add_stage(&mut self, id: String) {
        self.stage.insert(id, CharStageAddon::default());
    }
}

#[derive(Deserialize, Serialize)]
pub struct CharStoryAddon {
    pub fts: u64,
    pub rts: u64,
}

impl Default for CharStoryAddon {
    fn default() -> Self {
        let time = time(-1);
        Self { fts: time, rts: time }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharStageAddon {
    pub start_times: u8,
    pub complete_time: u8,
    pub state: u8,
    pub fts: u64,
    pub rts: u64,
    pub start_time: u8,
}

impl Default for CharStageAddon {
    fn default() -> Self {
        let time = time(-1);
        Self {
            start_times: 0,
            complete_time: 1,
            state: 3,
            fts: time,
            rts: time,
            start_time: 2,
        }
    }
}
