use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipTable {
    pub equip_dict: HashMap<String, UniEquip>,
}

/// Also known as Modules
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniEquip {
    pub char_id: String,
}

impl LoadTable for EquipTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/uniequip_table.json")?)?)
    }
}

#[derive(Deserialize, Serialize)]
pub struct BattleEquipTable {
    #[serde(flatten)]
    pub battle_equip_dict: HashMap<String, BattleUniEquip>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleUniEquip {
    pub phases: Vec<BattleUniEquipPhase>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleUniEquipPhase {}

impl LoadTable for BattleEquipTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/battle_equip_table.json")?)?)
    }
}
