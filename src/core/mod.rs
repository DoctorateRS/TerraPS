use axum::Json;
use chrono::Utc;
use serde_json::json;

use crate::{
    constants::config::CONFIG_JSON_PATH,
    utils::json::{read_json, JSON},
};

pub mod asset;
pub mod prod;
pub mod user;

pub async fn general_v1_server_time() -> JSON {
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
    let faketime_enabled = read_json(CONFIG_JSON_PATH)["userConfig"]["faketime"].as_i64().unwrap_or(-1);
    if faketime_enabled == -1 {
        Utc::now().timestamp() as u64
    } else {
        faketime_enabled as u64
    }
}
