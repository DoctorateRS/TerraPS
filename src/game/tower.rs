use crate::{
    constants::tower::TOWERDATA_PATH,
    utils::{enumerate, json::read_json},
};
use anyhow::{Error, Result};

fn current_coords(stage_id: &str) -> Result<u64> {
    let tower = read_json(TOWERDATA_PATH);

    for (index, layer) in enumerate(tower["tower"]["current"]["layer"].clone().as_array().unwrap()) {
        if layer["id"].as_str().unwrap() == stage_id {
            return Ok(index as u64);
        } else {
            continue;
        }
    }
    Err(Error::msg("Stage not found."))
}

fn create_recruit_list() {}
