use serde_json::{json, Value};

use crate::constants::url::RL_TABLE_URL;

use super::{game::update_data, json::get_keys};

pub async fn process_buff(mut rl_data: Value, buff_data: Value) -> Value {
    // TODO: Implements RLV2 codes
    rl_data
}
