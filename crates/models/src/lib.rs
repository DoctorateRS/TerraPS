use serde::{Deserialize, Serialize};

pub mod account;
pub mod gamemode;
mod misc;
pub mod online;
pub mod payload;
pub mod prod;
pub mod response;
pub mod social;
pub mod tables;

#[cfg(test)]
mod test;

pub use misc::*;

/// Represent an empty Object.
#[derive(Serialize, Deserialize, Default)]
pub struct NullObj {}
/// Represent an empty Array.
type NullVec = [u8; 0];

impl NullObj {
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
    modified: NullObj,
    deleted: NullObj,
}

impl PlayerDataDeltaStatic {
    pub const fn default() -> Self {
        Self {
            pdt: PddInner { modified: NullObj {}, deleted: NullObj {} },
        }
    }
}
