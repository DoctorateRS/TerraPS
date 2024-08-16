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
pub mod npc;
pub mod skin;
pub mod social;
pub mod status;
pub mod sync;
pub mod theme;

use std::collections::HashMap;

use activity::ActivityEnum;
use campaignv2::CampaignV2;
use character::Troop;
use crisis::{Crisis, CrisisV2};
use dexnav::DexNav;
use dungeon::Dungeon;
use flag::PushFlags;
use mission::Mission;
use namecard::NameCardStyle;
use npc::NpcAudio;
use skin::Skin;
use social::Social;
use status::PlayerStatus;
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
    pub dungeon: Dungeon,
    pub activity: HashMap<String, ActivityEnum>,
    pub status: PlayerStatus,
    pub name_card_style: NameCardStyle,
    pub troop: Troop,
    pub npc_audio: HashMap<String, NpcAudio>,
    pub push_flags: PushFlags,
    pub equipment: EmptyMap,
    pub skin: Skin,
    pub mission: Mission,
    pub social: Social,
    pub dex_nav: DexNav,
    pub crisis: Crisis,
    pub crisis_v2: CrisisV2,
    pub campaign_v2: CampaignV2,
    pub inventory: EmptyMap,
}
