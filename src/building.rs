use axum::Json;
use serde_json::{json, Number, Value};

use crate::{
    constants::{self, url::BUILDING_TABLE_URL, user::BUILDING_JSON_PATH},
    core::JSON,
    utils::{read_json, update_data, write_json},
};

fn update_building_char_inst_id_list(building_data: Value) -> Value {
    let mut building_data = building_data.clone();
    for (char_inst_id, _) in building_data["chars"].clone().as_object().unwrap() {
        building_data["chars"][char_inst_id]["roomSlotId"] = Value::String("".to_string());
        building_data["chars"][char_inst_id]["index"] = Value::Number(Number::from(-1));
    }
    for (room_slot, _) in building_data["roomSlots"].clone().as_object().unwrap() {
        for (char_inst_id, char_data) in building_data["roomSlots"][&room_slot]["charInstIds"].clone().as_object().unwrap() {
            building_data["chars"][char_inst_id]["roomSlotId"] = Value::String(room_slot.to_string());
            building_data["chars"][char_inst_id]["index"] = Value::Number(Number::from(char_data["index"].as_i64().unwrap()));
        }
    }
    building_data
}

pub async fn building_sync() -> JSON {
    let mut building_data = read_json(constants::user::BUILDING_JSON_PATH);
    let user_data = read_json(constants::user::USER_JSON_PATH);
    let mut chars = json!({});
    for (char_inst_id, _) in user_data["user"]["troop"]["chars"].as_object().unwrap() {
        chars[char_inst_id] = json!({ "charId": user_data["user"]["troop"]["chars"][char_inst_id]["charId"],
            "lastApAddTime": 1695000000,
            "ap": 8640000,
            "roomSlotId": "",
            "index": -1,
            "changeScale": 0,
            "bubble": {
                "normal": {
                    "add": -1,
                    "ts": 0
                },
                "assist": {
                    "add": -1,
                    "ts": 0
                }
            },
            "workTime": 0
        })
    }
    building_data["chars"] = chars;
    let mut building_data = update_building_char_inst_id_list(building_data);
    let building_table = update_data(BUILDING_TABLE_URL).await;
    let mut furniture = json!({});
    for (furn_index, _) in building_table["customData"]["furnitures"].as_object().unwrap() {
        furniture[furn_index] = json!({
            "count": 9999,
            "inUse": 0
        })
    }
    building_data["furniture"] = furniture;
    write_json(BUILDING_JSON_PATH, building_data.clone());
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "building": building_data
            },
            "deleted": {}
        }
    }))
}
