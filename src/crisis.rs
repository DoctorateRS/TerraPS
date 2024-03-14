use axum::{extract::Json, response::Response};
use serde_json::{json, Value};

use crate::{
    datapath::{CONFIG_JSON_PATH, CRISIS_V2_JSON_BASE_PATH, RUNE_JSON_PATH},
    utils::{read_json, write_json},
};

pub fn crisis_v2_get_info() -> Response<Value> {
    let crisis_v2_json = match read_json(CONFIG_JSON_PATH).unwrap()["crisisV2Config"]["selectedCrisis"].as_str() {
        Some(crisis) => read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{crisis}.json").as_str()).unwrap(),
        None => {
            json!({"info": {}, "ts": 1700000000, "playerDataDelta": {"modified": {}, "deleted": {}}})
        }
    };

    Response::builder().status(200).body(crisis_v2_json).unwrap()
}

pub fn crisis_v2_battle_start(Json(request_data): Json<Value>) -> Response<Value> {
    let battle_data = json!({ "mapId": request_data["mapId"], "runeSlots": request_data["runeSlots"] });
    write_json(RUNE_JSON_PATH, battle_data).unwrap();
    Response::builder()
        .status(200)
        .body(json!({
            "result": 0,
            "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
            "playerDataDelta": {"modified": {}, "deleted": {}},
        }))
        .unwrap()
}

pub fn crisis_v2_battle_finish() {
    let battle_data = read_json(RUNE_JSON_PATH).unwrap();
    let mapId = battle_data["mapId"].to_string();
    let _runeSlots = battle_data["runeSlots"].to_string();
    let _score: [u32; 6] = [0, 0, 0, 0, 0, 0];
    let selected_crisis = read_json(CONFIG_JSON_PATH).unwrap()["crisisV2Config"]["selectedCrisis"].to_string();
    let rune = read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str()).unwrap();

    let _node = json!({});
    for _slot in rune["info"]["mapDetailDataMap"][mapId]["nodeDataMap"].as_object().unwrap() {}
}
