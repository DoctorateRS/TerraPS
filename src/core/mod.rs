use axum::Json;
use serde_json::{json, Value};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type JSON = Json<Value>;

use crate::{constants, utils::json_utils::read_json};

pub mod asset;
pub mod prod;
pub mod user;

pub async fn general_v1_server_time() -> Json<Value> {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "serverTime": time(),
            "isHoliday": false
        }
    }))
}

pub fn time() -> u64 {
    let faketime_enabled = read_json(constants::config::CONFIG_JSON_PATH).clone()["userConfig"]["faketime"]
        .as_i64()
        .unwrap_or(-1);
    if faketime_enabled == -1 {
        chrono::Utc::now().timestamp() as u64
    } else {
        faketime_enabled as u64
    }
}
