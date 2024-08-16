use serde::{Deserialize, Serialize};

use crate::models::EmptyMap;

#[derive(Serialize, Deserialize)]
pub struct Mission {
    pub missions: Missions,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Missions {
    pub activity: EmptyMap,
}

pub const STATIC_MISSION: Mission = Mission { missions: Missions { activity: EmptyMap {} } };
