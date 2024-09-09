use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::{collections::HashMap, fs::File};

use super::LoadTable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SkinTable {
    pub char_skins: HashMap<String, CharSkin>,
}

impl LoadTable for SkinTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/skin_table.json")?)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharSkin {
    pub skin_id: String,
    pub char_id: String,
    pub display_skin: DisplaySkinTable,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySkinTable {
    pub get_time: u64,
    pub on_year: u16,
    pub on_period: u32,
}
