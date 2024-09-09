use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

use anyhow::{Error, Result};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterTable {
    #[serde(flatten)]
    pub table: HashMap<String, CharType>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum CharType {
    Character(Character),
    Other {},
}

impl LoadTable for CharacterTable {
    type Err = Error;

    fn load() -> Result<Self> {
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
