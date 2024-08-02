use serde::{Deserialize, Serialize};

pub mod account;
pub mod online;
pub mod payload;
pub mod prod;
pub mod social;

#[derive(Serialize, Deserialize)]
pub struct EmptyMap {}

impl EmptyMap {
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerDataDeltaStatic {
    #[serde(rename = "playerDataDelta")]
    pdt: PddInner,
}

#[derive(Serialize, Deserialize)]
struct PddInner {
    modified: EmptyMap,
    deleted: EmptyMap,
}

impl PlayerDataDeltaStatic {
    pub fn default() -> Self {
        Self {
            pdt: PddInner { modified: EmptyMap {}, deleted: EmptyMap {} },
        }
    }
}
