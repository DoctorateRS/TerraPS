use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants::{
        config::CONFIG_JSON_PATH,
        user::{CRISIS_V2_JSON_BASE_PATH, RUNE_JSON_PATH},
    },
    core::time,
    utils::{read_json, write_json},
};

pub async fn crisis_v2_get_info() -> Json<Value> {
    let config = read_json(CONFIG_JSON_PATH);
    let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap_or("cc2");
    Json(read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str()))
}

pub async fn crisis_v2_battle_start(Json(payload): Json<Value>) -> Json<Value> {
    let battle_data = json!({
        "mapId": payload["mapId"],
        "runeSlots": payload["runeSlots"]
    });
    write_json(RUNE_JSON_PATH, battle_data);
    Json(json!({
        "result": 0,
        "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn crisis_v2_battle_finish(Json(payload): Json<Value>) -> Json<Value> {
    let config = read_json(CONFIG_JSON_PATH);
    let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap_or("cc2");
    let rune = read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str());
    let battle_data = read_json(RUNE_JSON_PATH).clone();
    let map_id = battle_data["mapId"].as_str().unwrap_or("crisis_v2_02-01");
    let rune_slots = battle_data["runeSlots"].as_array().unwrap();
    let score_current = [0, 0, 0, 0, 0, 0];
    let mut nodes = json!({});
    let slots = match rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"].as_array() {
        Some(slots) => slots,
        None => panic!("Invalid mapId."),
    };
    for slot in slots {
        if !slot.as_str().unwrap().starts_with("node_") {
            continue;
        } else {
            let slot_id = slot.as_str().unwrap();
            let slot_data = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][slot_id].clone();
        }
    }
    Json(nodes)
}
