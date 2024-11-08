use std::fs::File;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharmTable {
    pub charm_list: Vec<Charm>,
}

impl LoadTable for CharmTable {
    type Err = Error;

    fn load() -> Result<Self> {
        Ok(from_reader(File::open("../../data/excel/charm_table.json")?)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Charm {
    pub id: String,
}
