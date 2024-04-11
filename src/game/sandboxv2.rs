use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants::{
        sandbox::{SANDBOX_JSON_PATH, SANDBOX_TEMP_JSON_PATH},
        templates::SANDBOX_TEMPLATE,
        url::SANDBOX_TABLE_URL,
    },
    utils::{
        game::update_data,
        json::{get_keys, read_json, write_json, JSON},
    },
};

pub async fn create_game() -> JSON {
    let sandbox = read_json(SANDBOX_TEMPLATE);
    write_json(SANDBOX_JSON_PATH, &sandbox);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}

pub async fn set_squad(Json(payload): JSON) -> JSON {
    let mut sandbox_data = read_json(SANDBOX_JSON_PATH);

    let index = match &payload["index"] {
        Value::Number(index) => index.as_u64().unwrap().to_string(),
        Value::String(index) => index.as_str().to_string(),
        _ => panic!("Invalid index"),
    };

    sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["troop"]["squad"][&index] = json!({
        "slots": payload["slots"],
        "tools": payload["tools"]
    });

    write_json(SANDBOX_JSON_PATH, &sandbox_data);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox_data
            },
            "deleted": {}
        }
    }))
}

pub async fn sandbox_battle_start(Json(payload): JSON) -> JSON {
    let mut sandbox_temp = read_json(SANDBOX_TEMP_JSON_PATH);
    sandbox_temp["currentNodeId"] = payload["nodeId"].clone();
    write_json(SANDBOX_TEMP_JSON_PATH, &sandbox_temp);
    Json(json!({
        "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
        "isEnemyRush": false,
        "shinyAnimal": {},
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn sandbox_battle_finish(Json(payload): JSON) -> JSON {
    let sandbox_temp = read_json(SANDBOX_TEMP_JSON_PATH);
    let mut sandbox_data = read_json(SANDBOX_JSON_PATH);
    let node_id = sandbox_temp["currentNodeId"].as_str().unwrap();
    if !get_keys(&sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][&node_id]).contains(&String::from("building")) {
        sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][&node_id]["building"] = json!([]);
    }
    let mut building = sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][&node_id]["building"]
        .as_array()
        .unwrap()
        .clone();
    let placed_items = payload["sandboxV2Data"]["placedItems"].clone();
    for keys in get_keys(&placed_items) {
        if placed_items[&keys]["value"].get("hpRatio").is_some() {
            building.push(json!({
                "key": &placed_items[&keys]["key"]["itemId"],
                "pos": [
                    &placed_items[&keys]["key"]["position"]["row"],
                    &placed_items[&keys]["key"]["position"]["col"]
                ],
                "hpRatio": 10000,
                "dir": &placed_items[&keys]["value"]["direction"]
            }))
        } else {
            for building_data in &building {
                if building_data["pos"].as_array().unwrap()[0].as_i64().unwrap()
                    == placed_items[&keys]["key"]["position"]["row"].as_i64().unwrap()
                    && building_data["pos"].as_array().unwrap()[1].as_i64().unwrap()
                        == placed_items[&keys]["key"]["position"]["col"].as_i64().unwrap()
                {
                    building.pop();
                    break;
                }
            }
        }
    }
    sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][&node_id]["building"] = json!(building);
    write_json(SANDBOX_JSON_PATH, &sandbox_data);
    Json(json!({
        "success": true,
        "rewards": [],
        "randomRewards": [],
        "costItems": [],
        "isEnemyRush": false,
        "enemyRushCount": [],
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": &sandbox_data
            },
            "deleted": {}
        }
    }))
}

pub async fn home_build_save(Json(payload): JSON) -> JSON {
    let sandbox_table = update_data(SANDBOX_TABLE_URL).await;

    Json(json!({}))
}
