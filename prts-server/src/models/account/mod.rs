pub mod activity;
pub mod avatar;
pub mod background;
pub mod campaignv2;
pub mod character;
pub mod crisis;
pub mod dexnav;
pub mod dungeon;
pub mod flag;
pub mod gamemode;
pub mod mainline;
pub mod medal;
pub mod mission;
pub mod namecard;
pub mod skin;
pub mod social;
pub mod status;
pub mod sync;
pub mod theme;

use campaignv2::CampaignV2;
use dungeon::Dungeon;
pub use sync::*;

use serde::{Deserialize, Serialize};

use super::EmptyMap;

#[derive(Serialize, Deserialize)]
pub struct AccountSyncData {
    result: u8,
    ts: u64,
    user: User,
}

/// Userdata.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    dungeon: Dungeon,

    campaign_v2: CampaignV2,
    inventory: EmptyMap,
}
