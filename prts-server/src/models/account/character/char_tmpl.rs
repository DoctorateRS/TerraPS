use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::chara::{Equip, Skill};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CharTmpl {
    pub skin_id: String,
    pub default_skill_index: i8,
    pub skills: Vec<Skill>,
    pub current_equip: Option<String>,
    pub equip: HashMap<String, Equip>,
}

impl CharTmpl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_skin(&mut self, skin: String) {
        self.skin_id = skin
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
}
