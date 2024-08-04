use serde::{Deserialize, Serialize};

use crate::models::EmptyMap;

#[derive(Serialize, Deserialize)]
pub struct Mission {
    missions: Missions,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
struct Missions {
    activity: EmptyMap,
}
