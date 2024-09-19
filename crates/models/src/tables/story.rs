use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::NullObj;

use super::LoadTable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryTable {
    #[serde(flatten)]
    pub stories: HashMap<String, NullObj>,
}

impl LoadTable for StoryTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/story_table.json")?)?)
    }
}
