use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookInfoTable {
    pub handbook_dict: HashMap<String, HandbookInfo>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookInfo {
    pub handbook_avg_list: Vec<HandbookStory>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookStory {
    pub story_set_id: String,
}

impl LoadTable for HandbookInfoTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/handbook_info_table.json")?)?)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandbookEnemyTable {}
