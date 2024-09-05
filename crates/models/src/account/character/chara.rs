use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common_utils::time;

use super::char_tmpl::CharTmpl;

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceLan {
    #[default]
    Jp,
    En,
    Cn,
    Kr,
    #[serde(untagged)]
    Other(String),
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Char {
    pub inst_id: u32,
    pub char_id: String,
    favor_point: u16,
    potential_rank: u8,
    pub main_skill_lvl: u8,
    pub skin: String,
    pub level: u8,
    exp: u32,
    pub evolve_phase: u8,
    pub default_skill_index: i8,
    gain_time: u64,
    pub skills: Vec<Skill>,
    pub voice_lan: VoiceLan,
    pub current_equip: Option<String>,
    pub equip: HashMap<String, Equip>,
    pub star_mark: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    current_tmpl: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    tmpl: HashMap<String, CharTmpl>,
}

impl Char {
    /// Construct a new character from a charId string.
    pub fn new(char_id: String) -> Self {
        let inst_id = char_id.split('_').nth(1).unwrap_or("").parse().unwrap_or(0);
        let skin = char_id.clone() + "#1";
        Self {
            inst_id,
            char_id,
            favor_point: 25570,
            potential_rank: 5,
            main_skill_lvl: 7,
            skin,
            level: 0,
            exp: 0,
            evolve_phase: 0,
            default_skill_index: -1,
            gain_time: time(-1),
            skills: Vec::new(),
            voice_lan: VoiceLan::Jp,
            current_equip: None,
            equip: HashMap::new(),
            star_mark: 0,
            current_tmpl: None,
            tmpl: HashMap::new(),
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level
    }

    pub fn set_elite_status(&mut self, phase: u8) {
        self.evolve_phase = phase
    }

    pub fn set_skin(&mut self, skin: String) {
        self.skin = skin
    }

    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill)
    }

    pub fn set_default_skill_index(&mut self, index: i8) {
        self.default_skill_index = index
    }

    pub fn add_equip(&mut self, id: String, equip: Equip) {
        self.equip.insert(id, equip);
    }

    pub fn set_equip(&mut self, equip_id: String) {
        if equip_id.is_empty() {
            self.current_equip = None
        } else {
            self.current_equip = Some(equip_id)
        }
    }

    pub fn change_star_mark(&mut self) {
        if self.star_mark == 0 {
            self.star_mark = 1
        } else {
            self.star_mark = 0
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    id: String,
    unlock: u8,
    state: u8,
    level: u8,
    complete_time: i8,
}

impl Skill {
    pub fn new(id: String, specializable: bool) -> Self {
        if !specializable {
            Self {
                id,
                unlock: 1,
                state: 0,
                level: 3,
                complete_time: -1,
            }
        } else {
            Self {
                id,
                unlock: 1,
                state: 0,
                level: 0,
                complete_time: -1,
            }
        }
    }
}

/// AKA Module.
#[derive(Deserialize, Serialize)]
pub struct Equip {
    hide: u8,
    locked: u8,
    level: u8,
}

impl Equip {
    pub fn new(level: u8) -> Self {
        Self { hide: 0, locked: 0, level }
    }
}
