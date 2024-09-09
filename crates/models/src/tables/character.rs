use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

use anyhow::Result;
use serde_json::from_reader;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterTable {
    #[serde(flatten)]
    pub content: HashMap<String, Character>,
}

impl CharacterTable {
    pub fn load() -> Result<Self> {
        Ok(from_reader(File::open("../../data/excel/character_table.json")?)?)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub rarity: String,
    pub phases: Vec<CharacterPhase>,
    pub skills: Vec<CharacterSkill>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPhase {
    pub max_level: u8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSkill {
    pub skill_id: String,
}
