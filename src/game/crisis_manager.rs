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
        utils::{
            comp::max,
            json::{get_keys, read_json, write_json, JSON},
        },
    };
    pub async fn crisis_v2_get_info() -> JSON {
        let config = read_json(CONFIG_JSON_PATH);
        let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap_or("cc2");
        Json(read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str()))
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

        let selected_crisis = config["crisisV2Config"]["selectedCrisis"].as_str().unwrap();
        let rune = read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{selected_crisis}.json").as_str());
        let battle_data = read_json(RUNE_JSON_PATH).clone();
        let map_id = battle_data["mapId"].as_str().unwrap();
        let rune_slots = battle_data["runeSlots"].clone();

        let mut score_current = [0, 0, 0, 0, 0, 0];

        let mut nodes = json!({});

        for slot in get_keys(&rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"]) {
            let score;
            let rune_data;
            if !slot.starts_with("node_") {
                continue;
            }
            let slot_id = &slot;
            let node_data = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][&slot_id].clone();
            let slot_pack_id = node_data["slotPackId"].clone();
            if slot_pack_id.is_null() {
                continue;
            }
            let slot_pack_id = slot_pack_id.as_str().unwrap();
            if nodes.get(slot_pack_id).is_none() {
                nodes[&slot_pack_id] = json!({});
            }
            let mutual_exclusion_group = if node_data.get("mutualExclusionGroup").is_some() && !node_data["mutualExclusionGroup"].is_null() {
                node_data["mutualExclusionGroup"].as_str().unwrap()
            } else {
                &slot
            };
            if nodes[&slot_pack_id].get("mutualExclusionGroup").is_none() {
                nodes[&slot_pack_id][&mutual_exclusion_group] = json!({})
            }
            if node_data.get("runeId").is_some() {
                let r_id = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][&slot]["runeId"].clone();
                if !r_id.is_null() {
                    rune_data = rune["info"]["mapDetailDataMap"][&map_id]["runeDataMap"][r_id.as_str().unwrap()].clone();
                    score = rune_data["score"].clone().as_i64().unwrap();
                } else {
                    score = 0;
                }
            } else {
                score = 0;
            };
            nodes[&slot_pack_id][mutual_exclusion_group][slot] = Value::Number(Number::from(score));
        }

        let rune_slots = rune_slots
            .as_array()
            .unwrap()
            .iter()
            .map(|rune| rune.as_str().unwrap())
            .collect::<Vec<&str>>();

        // FIXME: FIX THIS
        for slot_pk_id in get_keys(&nodes) {
            let mut flag = true;
            for mutual_exclusion_group in get_keys(&nodes[&slot_pk_id]) {
                let mut score_max = 0;
                for slot in get_keys(&nodes[&slot_pk_id][&mutual_exclusion_group]) {
                    score_max = max(score_max, nodes[&slot_pk_id][&mutual_exclusion_group][slot].as_u64().unwrap())
                }
                let mut flag2 = false;
                for slot in get_keys(&nodes[&slot_pk_id][&mutual_exclusion_group]) {
                    if nodes[&slot_pk_id][&mutual_exclusion_group][&slot].as_u64().unwrap() != score_max {
                        continue;
                    }
                    if rune_slots.contains(&slot.as_str()) {
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
                let bag_data = rune["info"]["mapDetailDataMap"][&map_id]["bagDataMap"][&slot_pk_id].clone();
                score_current[bag_data["dimension"].as_u64().unwrap() as usize] += bag_data["rewardScore"].as_u64().unwrap();
            }
        }

        let mut rune_ids = Vec::new();

        for slot in &rune_slots {
            let node_data = rune["info"]["mapDetailDataMap"][&map_id]["nodeDataMap"][&slot].clone();
            if node_data.get("runeId").is_some() {
                let rune_id = node_data["runeId"].clone();
                rune_ids.push(rune_id.clone());
                let rune_data = rune["info"]["mapDetailDataMap"][&map_id]["runeDataMap"][rune_id.as_str().unwrap()].clone();
                score_current[rune_data["dimension"].as_u64().unwrap() as usize] += rune_data["score"].as_u64().unwrap();
            }
        }

        Json(json!({
            "result": 0,
            "mapId": map_id,
            "runeSlots": rune_slots,
            "runeIds": rune_ids,
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
}
