use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SquadSlot {
    pub char_inst_id: u32,
    pub skill_index: u8,
    pub current_equip: Option<String>,
}

impl SquadSlot {
    pub fn new(id: u32, skill_index: u8, current_equip: Option<String>) -> Self {
        Self { char_inst_id: id, skill_index, current_equip }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Squad {
    pub squad_id: String,
    pub name: String,
    pub slots: [Option<SquadSlot>; 12],
}

impl Squad {
    pub fn new(squad_id: u8, name: String) -> Self {
        Self {
            squad_id: squad_id.to_string(),
            name,
            slots: [None, None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name
    }

    pub fn set_char(&mut self, slot: SquadSlot, index: usize) {
        self.slots[index] = Some(slot)
    }
}
