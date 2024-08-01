use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod account;
pub mod online;
pub mod payload;
pub mod prod;
pub mod social;

#[derive(Serialize, Deserialize)]
pub struct PlayerDataDeltaStatic {
    #[serde(rename = "playerDataDelta")]
    pdt: PddInner,
}

#[derive(Serialize, Deserialize)]
struct PddInner {
    modified: HashMap<(), ()>,
    deleted: HashMap<(), ()>,
}

impl PlayerDataDeltaStatic {
    pub fn default() -> Self {
        Self {
            pdt: PddInner {
                modified: HashMap::<(), ()>::default(),
                deleted: HashMap::<(), ()>::default(),
            },
        }
    }
}
