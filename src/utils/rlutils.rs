use serde_json::{json, Value};

use crate::constants::url::RL_TABLE_URL;

use super::{game::update_data, json::get_keys};

pub async fn process_buff(mut rl_data: Value, buff_data: Value) -> Value {
    rl_data
}

async fn process_relics(mut rl_data: Value, relics: Vec<Value>) -> Value {
    let relics_table = update_data(RL_TABLE_URL).await["details"]["rogue_1"]["relics"].clone();

    for relic in relics {
        let mut relic_cnt = String::from("r_0");
        if !get_keys(&rl_data["current"]["inventory"]["relic"]).is_empty() {
            relic_cnt = format!(
                "r_{}",
                get_keys(&rl_data["current"]["inventory"]["relic"])
                    .last()
                    .unwrap()
                    .split("r_")
                    .collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap()
                    + 1
            );
        }
        let r_buffs = relics_table[&relic["id"].as_str().unwrap()]["buffs"].as_array().unwrap();

        for r_buff in r_buffs {
            match r_buff["key"].as_str().unwrap() {
                _ => (),
            }
        }
    }

    rl_data
}
