use axum::Json;
use serde_json::{json, Number, Value};

use crate::{
    constants::{
        url::BUILDING_TABLE_URL,
        user::{BUILDING_JSON_PATH, USER_JSON_PATH},
    },
    core::JSON,
    utils::{
        json_utils::{read_json, write_json},
        update_data,
    },
};

fn update_building_char_inst_id_list(building_data: Value) -> Value {
    let mut building_data = building_data.clone();
    for (char_inst_id, _) in building_data["chars"].clone().as_object().unwrap() {
        building_data["chars"][char_inst_id]["roomSlotId"] = Value::String("".to_string());
        building_data["chars"][char_inst_id]["index"] = Value::Number(Number::from(-1));
    }
    for (room_slot, _) in building_data["roomSlots"].clone().as_object().unwrap() {
        for key in building_data["roomSlots"][&room_slot]["charInstIds"].clone().as_array().unwrap() {
            if key.as_i64().unwrap() == -1 {
                continue;
            }
            let key = key.as_str().unwrap();
            building_data["chars"][key]["roomSlotId"] = Value::String(room_slot.to_string());
            building_data["chars"][key]["index"] = Value::Number(Number::from(
                building_data["roomSlots"][&room_slot]["charInstIds"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .position(|x| x == key)
                    .unwrap() as i64,
            ));
        }
    }
    building_data
}

pub async fn building_sync() -> JSON {
    let mut building_data = read_json(BUILDING_JSON_PATH);
    let user_data = read_json(USER_JSON_PATH);
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

pub async fn building_get_recent_visitors() -> JSON {
    Json(json!({"visitors": []}))
}

pub async fn building_get_info_share_visitor_num() -> JSON {
    Json(json!({"num": 0}))
}

pub async fn building_change_diy_solution(Json(payload): JSON) -> JSON {
    let room_slot_id = payload["roomSlotId"].as_str().unwrap();
    let diy_solution = payload["solution"].clone();

    let mut building_data = read_json(BUILDING_JSON_PATH);
    building_data["rooms"]["DORMITORY"][room_slot_id]["diySolution"] = diy_solution;
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

pub async fn building_assign_char(Json(payload): JSON) -> JSON {
    let room_slot_id = payload["roomSlotId"].as_str().unwrap();
    let char_inst_id_list = payload["charInstId"].as_array().unwrap();

    let mut building_data = read_json(BUILDING_JSON_PATH);
    for char_inst_id in char_inst_id_list {
        let char_inst_id = char_inst_id.as_i64().unwrap();
        if char_inst_id == -1 {
            continue;
        }
        let char_inst_id = char_inst_id.to_string();
        if building_data["chars"][&char_inst_id]["index"].as_i64().unwrap() != -1 {
            let old_char_data = building_data["chars"][&char_inst_id].clone();
            let old_room_slot_id = old_char_data["roomSlotId"].as_str().unwrap();
            let old_index = old_char_data["index"].as_i64().unwrap();
            building_data["roomSlots"][old_room_slot_id]["charInstIds"][old_index as usize] = Value::Number(Number::from(-1));
        }
    }
    building_data["roomSlots"][room_slot_id]["charInstIds"] = json!(char_inst_id_list);
    building_data = update_building_char_inst_id_list(building_data);
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

pub async fn building_set_building_assist(Json(payload): JSON) -> JSON {
    let assist_type = payload["type"].as_str().unwrap();
    let char_inst_id = payload["charInstId"].clone();
    let mut building_data = read_json(BUILDING_JSON_PATH);
    building_data["assist"][&assist_type]["charInstId"] = char_inst_id;
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

pub async fn building_get_assist_report() -> JSON {
    Json(json!({
        "reports": [
        ],
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}
