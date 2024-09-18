use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookInfoTable {
    pub handbook_dict: HashMap<String, HandbookInfoStory>,
    pub handbook_stage_data: HashMap<String, HandbookInfoStage>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookInfoStory {
    pub handbook_avg_list: Vec<HandbookStory>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookStory {
    pub story_set_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookInfoStage {
    pub stage_id: String,
}

impl LoadTable for HandbookInfoTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/handbook_info_table.json")?)?)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookEnemyTable {
    pub enemy_data: HashMap<String, HandbookEnemy>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookEnemy {}

impl LoadTable for HandbookEnemyTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/enemy_handbook_table.json")?)?)
    }
}
