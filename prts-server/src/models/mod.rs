use serde::{Deserialize, Serialize};

pub mod account;
pub mod online;
pub mod payload;
pub mod prod;
pub mod social;

#[derive(Serialize, Deserialize, Default)]
pub struct EmptyMap {}

impl EmptyMap {
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerDataDeltaStatic {
    #[serde(rename = "playerDataDelta")]
    pdt: PddInner,
}

#[derive(Serialize, Deserialize, Default)]
struct PddInner {
    modified: EmptyMap,
    deleted: EmptyMap,
}

impl PlayerDataDeltaStatic {
    pub const fn default() -> Self {
        Self {
            pdt: PddInner { modified: EmptyMap {}, deleted: EmptyMap {} },
        }
    }
}
