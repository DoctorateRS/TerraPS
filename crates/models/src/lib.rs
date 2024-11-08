use serde::{Deserialize, Serialize};

pub mod account;
pub mod building;
pub mod delta;
pub mod gamemode;
pub mod mail;
mod misc;
pub mod online;
pub mod pay;
pub mod payload;
pub mod prod;
pub mod social;
pub mod tables;

#[cfg(test)]
mod test;

pub use misc::*;

/// Represent an empty Object.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct NullObj {}

/// Represent an empty Array.
pub type NullVec = [u8; 0];

impl NullObj {
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct PlayerDataDelta {
    modified: NullObj,
    deleted: NullObj,
}

impl PlayerDataDelta {
    pub const fn default() -> Self {
        Self {
            modified: NullObj {},
            deleted: NullObj {},
        }
    }
}
