pub mod crisis {
    // Obsolete
}

pub mod crisis_v2 {
    use axum::Json;
    use serde_json::{json, Number, Value};

    use crate::{
        constants::{
            config::CONFIG_JSON_PATH,
            user::{CRISIS_V2_JSON_BASE_PATH, RUNE_JSON_PATH},
        },
        core::JSON,
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

    pub async fn crisis_v2_battle_finish() -> JSON {
        let config = read_json(CONFIG_JSON_PATH);
        let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap_or("cc2");
        let rune = read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str());
        let battle_data = read_json(RUNE_JSON_PATH).clone();
        let map_id = battle_data["mapId"].as_str().unwrap_or("crisis_v2_02-01");
        let rune_slots = battle_data["runeSlots"].clone();
        let mut score_current = [0, 0, 0, 0, 0, 0];
        let mut nodes = json!({});
        let slots = match rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"].as_array() {
            Some(slots) => slots,
            None => panic!("Invalid mapId."),
        };
        for slot in slots {
            let score;
            let rune_data;
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
        for (slot_pack_id, _) in nodes.as_object().unwrap() {
            let mut flag = true;
            for (mutual_exclusion_group, _) in nodes[slot_pack_id].as_object().unwrap() {
                let mut score_max = 0;
                for (slot, _) in nodes[slot_pack_id][mutual_exclusion_group].as_object().unwrap() {
                    score_max = max(score_max, nodes[slot_pack_id][mutual_exclusion_group][slot].as_i64().unwrap());
                }
                let mut flag2 = false;
                for (slot, _) in nodes[slot_pack_id][mutual_exclusion_group].as_object().unwrap() {
                    if nodes[slot_pack_id][mutual_exclusion_group][slot].as_i64().unwrap() != score_max {
                        continue;
                    };
                    if slots.get(slot.as_str()).is_some() {
                        flag2 = true;
                        break;
                    }
                }
                if !flag2 {
                    flag = false;
                    break;
                }
            }
            if flag {
                let bag_data = rune["info"]["mapDetailDataMap"][&map_id]["bagDataMap"][&slot_pack_id].clone();
                let mut dimension = bag_data["dimension"].as_u64().unwrap();
                if dimension > 5 {
                    dimension = 5;
                }
                score_current[dimension as usize] += bag_data["rewardScore"].as_u64().unwrap();
            }
        }

        for (slot, _) in rune_slots.as_object().unwrap() {
            let node_data = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][&slot].clone();
            if node_data.get("runeId").is_some() {
                let rune_id = node_data["runeId"].clone();
                let rune_data = rune["info"]["mapDetailDataMap"][&map_id]["runeDataMap"][rune_id.as_str().unwrap()].clone();
                score_current[rune_data["dimension"].as_u64().unwrap() as usize] += rune_data["rewardScore"].as_u64().unwrap();
            }
        }

        Json(json!({
            "result": 0,
            "mapId": map_id,
            "runeSlots": rune_slots,
            "isNewRecord": false,
            "scoreRecord": [0, 0, 0, 0, 0, 0],
            "scoreCurrent": score_current,
            "runeCount": [0, 0],
            "commentNew": [],
            "commentOld": [],
            "ts": 1700000000,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
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

    fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }
}
