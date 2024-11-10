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

use std::{collections::HashMap, fs::File, path::Path};

use activity::ActivityEnum;
use anyhow::Result;
use avatar::Avatar;
use background::Background;
use campaignv2::CampaignV2;
use character::Troop;
use common_utils::time;
use crisis::{Crisis, CrisisV2};
use dexnav::DexNav;
use dungeon::Dungeon;
use flag::PushFlags;
use mainline::Mainline;
use medal::Medal;
use mission::{Mission, STATIC_MISSION};
use namecard::NameCardStyle;
use npc::NpcAudio;
use serde_json::from_reader;
use skin::Skin;
use social::{Social, SOCIAL_STATIC};
use status::PlayerStatus;

use serde::{Deserialize, Serialize};
use theme::HomeTheme;

use crate::PlayerDataDelta;

use super::NullObj;

#[derive(Serialize, Deserialize)]
pub struct AccountSyncData {
    result: u8,
    ts: u64,
    user: User,
}

#[derive(Serialize, Deserialize)]
pub struct Backflow {
    open: bool,
    current: Option<String>,
}

const BACKFLOW: Backflow = Backflow { open: true, current: None };

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub result: u8,
    pub ts: u64,
    pub user: User,
    pub player_data_delta: PlayerDataDelta,
}

impl UserData {
    /// Load the Userdata from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(from_reader(File::open(path)?)?)
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            result: 0,
            ts: time(-1),
            user: User::default(),
            player_data_delta: PlayerDataDelta::default(),
        }
    }
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
    equipment: NullObj,
    pub skin: Skin,
    pub mission: Mission,
    pub social: Social,
    pub dex_nav: DexNav,
    pub crisis: Crisis,
    pub crisis_v2: CrisisV2,
    backflow: Backflow,
    pub mainline: Mainline,
    pub avatar: Avatar,
    pub background: Background,
    pub medal: Medal,
    pub home_theme: HomeTheme,
    pub campaigns_v2: CampaignV2,
    pub inventory: NullObj,
}

impl Default for User {
    fn default() -> Self {
        Self {
            dungeon: Dungeon::new(),
            activity: HashMap::new(),
            status: PlayerStatus::new(),
            name_card_style: NameCardStyle::new(),
            troop: Troop::default(),
            npc_audio: HashMap::new(),
            push_flags: PushFlags::default(),
            equipment: NullObj {},
            skin: Skin::default(),
            mission: STATIC_MISSION,
            social: SOCIAL_STATIC,
            dex_nav: DexNav::default(),
            crisis: Crisis::default(),
            crisis_v2: CrisisV2::default(),
            backflow: BACKFLOW,
            mainline: Mainline::new(),
            avatar: Avatar::new(),
            background: Background::new(),
            medal: Medal::new(),
            home_theme: HomeTheme::new(),
            campaigns_v2: CampaignV2::new(),
            inventory: NullObj {},
        }
    }
}
