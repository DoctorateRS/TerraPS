pub mod activity;
pub mod character;
pub mod dungeon;
pub mod flag;
pub mod namecard;
pub mod skin;
pub mod status;
pub mod sync;

pub use sync::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountSyncData {
    result: u8,
    ts: u64,
    user: User,
}

/// The Big Data
#[derive(Serialize, Deserialize)]
pub struct User {}
