use std::{collections::HashMap, fs::File};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use super::LoadTable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetroTable {
    pub retro_act_list: HashMap<String, RetroAct>,
    pub retro_trail_list: HashMap<String, RetroTrail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetroAct {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetroTrail {
    pub trail_reward_list: Vec<RetroTrailReward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetroTrailReward {
    pub trail_reward_id: String,
}

impl LoadTable for RetroTable {
    type Err = Error;

    fn load() -> Result<Self, Self::Err> {
        Ok(from_reader(File::open("../../data/excel/retro_table.json")?)?)
    }
}
