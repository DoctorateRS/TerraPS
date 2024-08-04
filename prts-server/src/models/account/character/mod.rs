use std::{collections::HashMap, default};

use serde::{Deserialize, Serialize};

mod addon;
mod squad;
pub use squad::*;

use crate::utils::time::time;

#[derive(Deserialize, Default)]
pub enum VoiceLan {
    #[default]
    Jp,
    En,
    Cn,
    Kr,
}

impl Serialize for VoiceLan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Jp => serializer.serialize_str("JP"),
            Self::En => serializer.serialize_str("EN"),
            Self::Cn => serializer.serialize_str("CN"),
            Self::Kr => serializer.serialize_str("KR"),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Char {
    #[serde(rename = "instId")]
    inst_id: u32,
    #[serde(rename = "charId")]
    char_id: String,
    #[serde(rename = "favorPoint")]
    favor_point: u16,
    #[serde(rename = "potentialRank")]
    potential_rank: u8,
    #[serde(rename = "mainSkillLvl")]
    pub main_skill_level: u8,
    skin: String,
    pub level: u8,
    exp: u32,
    #[serde(rename = "evolvePhase")]
    pub evolve_phase: u8,
    #[serde(rename = "defaultSkillIndex")]
    default_skill_index: i8,
    #[serde(rename = "gainTime")]
    gain_time: u64,
    pub skills: Vec<Skill>,
    #[serde(rename = "voiceLan")]
    pub voice_lan: VoiceLan,
    #[serde(rename = "currentEquip")]
    pub current_equip: Option<String>,
    equip: HashMap<String, Equip>,
    #[serde(rename = "starMark")]
    star_mark: u8,
}

impl Char {
    pub fn new(char_id: String) -> Self {
        let inst_id = char_id.split('_').nth(1).unwrap_or("").parse().unwrap_or(0);
        let skin = char_id.clone() + "#1";
        Self {
            inst_id,
            char_id,
            favor_point: 25570,
            potential_rank: 5,
            main_skill_level: 7,
            skin,
            level: 0,
            exp: 0,
            evolve_phase: 0,
            default_skill_index: -1,
            gain_time: time(),
            skills: Vec::new(),
            voice_lan: VoiceLan::Jp,
            current_equip: None,
            equip: HashMap::new(),
            star_mark: 0,
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level
    }

    /// Elite status:
    /// ```
    ///     0 => Elite 0
    ///     1 => Elite 1
    ///     2 => Elite 2
    /// ```
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
pub struct Skill {
    #[serde(rename = "skillId")]
    id: String,
    unlock: u8,
    state: u8,
    #[serde(rename = "specializeLevel")]
    level: u8,
    #[serde(rename = "completeUpgradeTime")]
    complete_time: i8,
}

impl Skill {
    pub fn new(id: String, is_three_star: bool) -> Self {
        if !is_three_star {
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
