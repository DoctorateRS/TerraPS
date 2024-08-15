pub mod activity;
pub mod avatar;
pub mod background;
pub mod character;
pub mod crisis;
pub mod dexnav;
pub mod dungeon;
pub mod flag;
pub mod mainline;
pub mod medal;
pub mod mission;
pub mod namecard;
pub mod skin;
pub mod social;
pub mod status;
pub mod sync;
pub mod theme;

pub use sync::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountSyncData {
    result: u8,
    ts: u64,
    user: User,
}

/// Userdata.
#[derive(Serialize, Deserialize)]
pub struct User {}
