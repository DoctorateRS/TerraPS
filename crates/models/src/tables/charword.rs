use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharWordTable {
    pub char_default_type_dict: HashMap<String, String>,
}

impl LoadTable for CharWordTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/charword_table.json")?)?)
    }
}
