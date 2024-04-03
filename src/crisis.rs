use axum::Json;
use serde_json::{json, Number, Value};

use crate::{
    constants::{
        config::CONFIG_JSON_PATH,
        user::{CRISIS_V2_JSON_BASE_PATH, RUNE_JSON_PATH},
    },
    core::{time, JSON},
    utils::{read_json, write_json},
};

pub async fn crisis_v2_get_info() -> JSON {
    let config = read_json(CONFIG_JSON_PATH);
    let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap_or("cc2");
    Json(read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str()))
}

pub async fn crisis_v2_battle_start(Json(payload): JSON) -> JSON {
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

pub async fn crisis_v2_battle_finish(Json(payload): JSON) -> JSON {
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
        let mut score;
        let mut rune_data = json!({});
        if !slot.as_str().unwrap().starts_with("node_") {
            continue;
        }
        let slot_id = slot.as_str().unwrap();
        let node_data = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][slot_id].clone();
        let slot_pack_id = node_data["slotPackId"].clone();
        if slot_pack_id.is_null() {
            continue;
        }
        if nodes.get(slot_pack_id.as_str().unwrap()).is_none() {
            nodes[slot_pack_id.as_str().unwrap()] = json!({});
        }
        let mutual_exclusion_group = if node_data.get("mutualExclusionGroup").is_some() && !node_data["mutualExclusionGroup"].is_null() {
            node_data["mutualExclusionGroup"].clone()
        } else {
            slot.clone()
        };
        if nodes[slot_id].get("mutualExclusionGroup").is_none() {
            nodes[slot_id][mutual_exclusion_group.as_str().unwrap()] = json!({})
        }
        if node_data.get("runeId").is_some() {
            let r_id = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][&slot.as_str().unwrap()]["runeId"].clone();
            if !r_id.is_null() {
                rune_data = rune["info"]["mapDetailDataMap"][&map_id]["runeDataMap"][r_id.as_str().unwrap()].clone();
                score = rune_data["score"].clone().as_i64().unwrap();
            } else {
                score = 0;
            }
        } else {
            score = 0;
        };
        nodes[slot_pack_id.as_str().unwrap()][mutual_exclusion_group.as_str().unwrap()][slot.as_str().unwrap()] =
            Value::Number(Number::from(score));
    }

    let slots = rune_slots.clone();
    for slot_pack_id in nodes.as_array() {}
    Json(nodes)
}

pub async fn crisis_v2_get_snapshot() -> JSON {
    Json(json!({
        "detail": {},
        "simple": {},
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}
