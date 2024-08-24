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
pub mod char_rlv2;

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Troop {
    cur_char_inst_id: u32,
    cur_squad_count: u8,
    squads: HashMap<String, Squad>,
    chars: HashMap<String, Char>,
    char_group: HashMap<String, CharGroupComponent>,
    char_mission: HashMap<String, HashMap<String, CharMissionState>>,
    addon: HashMap<String, CharAddon>,
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
    favor_point: u16,
}

impl Default for CharGroupComponent {
    fn default() -> Self {
        Self { favor_point: 25570 }
    }
}
