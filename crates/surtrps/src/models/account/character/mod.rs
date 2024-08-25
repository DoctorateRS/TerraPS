use std::collections::HashMap;

use addon::CharAddon;
use chara::Char;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use squad::Squad;

pub mod addon;
pub mod char_tmpl;
pub mod chara;
pub mod squad;

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Troop {
    pub cur_char_inst_id: u32,
    pub cur_squad_count: u8,
    pub squads: HashMap<String, Squad>,
    pub chars: HashMap<String, Char>,
    pub char_group: HashMap<String, CharGroupComponent>,
    pub char_mission: HashMap<String, HashMap<String, CharMissionState>>,
    pub addon: HashMap<String, CharAddon>,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum CharMissionState {
    Uncompleted = 0,
    Fulfilled = 1,
    Completed = 2,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharGroupComponent {
    pub favor_point: u16,
}

impl Default for CharGroupComponent {
    fn default() -> Self {
        Self { favor_point: 25570 }
    }
}
