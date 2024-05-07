use anyhow::{Ok, Result};
use serde_json::{json, Value};

use crate::constants::url::RL_TABLE_URL;

use super::{game::update_data, json::get_keys};

fn pop_recruit_dict() -> Value {
    json!({
        "0": 0,
        "1": 0,
        "2": 0,
        "3": 2,
        "4": 3,
        "5": 6,
    })
}

fn pop_upgrade_dict() -> Value {
    json!({
        "0": 0,
        "1": 0,
        "2": 0,
        "3": 1,
        "4": 2,
        "5": 3,
    })
}

fn node_type_dict() -> Value {
    json!({
        "Normal": 1,
        "Emergency": 2,
        "Trader": 8,
        "Rest": 16,
        "Encounter": 32,
        "Boon": 64,
        "Entertainment": 128
    })
}

pub async fn process_buff(rl_data: Value, buff_data: Value) -> Value {
    let items = buff_data["items"].clone();
    if buff_data.get("relic").is_some() {
        process_relic(rl_data, items).await.unwrap()
    } else {
        update_prop(rl_data, items).await.unwrap()
    }
}

async fn process_relic(rldata: Value, relics: Value) -> Result<Value> {
    let relic_table = update_data(RL_TABLE_URL).await["details"]["rogue_1"]["relics"].clone();

    for relic in get_keys(&relics) {
        let mut relic_cnt = String::from("r_0");
        if !get_keys(&rldata["current"]["inventory"]["relic"]).is_empty() {
            let cnt = 0;
            relic_cnt = format!("r_{}", cnt);
        }
    }

    Ok(rldata)
}

async fn update_prop(rldata: Value, update_items: Value) -> Result<Value> {
    Ok(rldata)
}
