use serde::{Deserialize, Serialize};

use crate::models::EmptyMap;

#[derive(Serialize, Deserialize)]
pub struct Mission {
    missions: Missions,
}

#[derive(Serialize, Deserialize)]
struct Missions {
    #[serde(rename = "ACTIVITY")]
    act: EmptyMap,
}

impl Mission {
    pub fn new() -> Self {
        Self { missions: Missions { act: EmptyMap {} } }
    }
}
