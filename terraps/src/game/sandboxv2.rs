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
        json::{get_keys, JSON},
    },
};

use common_utils::{read_json, write_json};

pub async fn create_game() -> JSON {
    let sandbox = read_json(SANDBOX_TEMPLATE);
    write_json(SANDBOX_JSON_PATH, &sandbox).unwrap_or(());
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

    let index = payload["index"].as_u64().unwrap() as usize;

    sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["troop"]["squad"][&index] = json!({
        "slots": payload["slots"],
        "tools": payload["tools"]
    });

    write_json(SANDBOX_JSON_PATH, &sandbox_data).unwrap_or(());
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

    sandbox_temp["currentNodeId"] = json!(payload["nodeId"]);

    write_json(SANDBOX_TEMP_JSON_PATH, &sandbox_temp).unwrap_or(());
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
    write_json(SANDBOX_JSON_PATH, &sandbox_data).unwrap_or(());
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
    let mut sandbox = read_json(SANDBOX_JSON_PATH);
    let node_id = payload["nodeId"].as_str().unwrap();

    if sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]
        .get("building")
        .is_none()
    {
        sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"] = json!([]);
    }

    for op in payload["operation"].as_array().unwrap() {
        match op["type"].as_i64().unwrap() {
            1 => sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "key": op["buildingId"],
                    "pos": [op["pos"]["row"], op["pos"]["col"]],
                    "hpRatio": 10000,
                    "dir": op["dir"],
                })),
            3 => {
                for build in sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                    .as_array()
                    .unwrap()
                {
                    if build["pos"][0] == op["pos"]["row"] && build["pos"][1] == op["pos"]["col"] {
                        sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                            .as_array_mut()
                            .unwrap()
                            .remove(0);
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    write_json(SANDBOX_JSON_PATH, &sandbox).unwrap_or(());

    Json(json!({"playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}

pub async fn settle_game() -> JSON {
    let sandbox = read_json(SANDBOX_JSON_PATH);

    let mut data = json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": {
                    "template": {
                        "SANDBOX_V2": {
                            "sandbox_1": {
                                "base": {
                                    "baseLv": 0,
                                    "upgradeProgress": [],
                                    "trapLimit": {},
                                    "portableUnlock": false,
                                    "outpostUnlock": false,
                                    "repairDiscount": 0,
                                    "bossKill": [],
                                },
                                "main": {
                                    "game": {
                                        "mapId": "",
                                        "day": 0,
                                        "maxDay": 0,
                                        "ap": 0,
                                        "maxAp": 0,
                                    },
                                    "map": {"season": {"type": 0, "remain": 0, "total": 0}},
                                    "report": {"settle": null, "daily": null},
                                    "event": {"node": {}, "effect": []},
                                },
                                "buff": {"rune": {"global": [], "node": {}, "char": {}}},
                                "riftInfo": {
                                    "isUnlocked": false,
                                    "randomRemain": 0,
                                    "difficultyLvMax": -1,
                                    "teamLv": 0,
                                    "fixFinish": [],
                                    "reservation": null,
                                    "gameInfo": null,
                                    "settleInfo": null,
                                },
                                "quest": {"pending": [], "complete": []},
                                "mission": {"squad": []},
                                "troop": {
                                    "food": {},
                                    "squad": [
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                        {"slots": [], "tools": []},
                                    ],
                                    "usedChar": [],
                                },
                                "cook": {
                                    "drink": 0,
                                    "extraDrink": 0,
                                    "book": {},
                                    "food": {},
                                },
                                "bag": {"material": {}, "craft": []},
                                "tech": {"token": 8, "cent": 0, "unlock": []},
                                "bank": {"book": [], "coin": {}},
                                "archive": {
                                    "save": [],
                                    "nextLoadTs": 0,
                                    "loadTimes": 0,
                                    "loadTs": 0,
                                },
                                "supply": {
                                    "unlock": false,
                                    "enable": false,
                                    "slotCnt": 0,
                                    "char": [],
                                },
                                "shop": {"unlock": false, "day": 0, "slots": []},
                                "status": {
                                    "ver": 1,
                                    "state": 0,
                                    "ts": 0,
                                    "isRift": false,
                                    "isGuide": false,
                                    "exploreMode": false,
                                },
                            }
                        }
                    }
                }
            },
            "deleted": {
                "sandboxPerm": {
                    "template": {
                        "SANDBOX_V2": {
                            "sandbox_1": {
                                "main": {
                                    "map": {"node": []},
                                    "stage": {"node": []},
                                    "enemy": {"enemyRush": []},
                                },
                                "buff": {
                                    "rune": {
                                        "node": [],
                                        "char": [],
                                    }
                                },
                                "troop": {"food": []},
                            }
                        }
                    }
                }
            },
        }
    });

    for node in get_keys(&sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["map"]["node"]) {
        data["playerDataDelta"]["deleted"]["sandboxPerm"]["SANDBOX_V2"]["sandbox_1"]["main"]["map"]["node"]
            .as_array_mut()
            .unwrap()
            .push(json!(node));
    }
    for node in get_keys(&sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"]) {
        data["playerDataDelta"]["deleted"]["sandboxPerm"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"]
            .as_array_mut()
            .unwrap()
            .push(json!(node));
    }
    for rush in get_keys(&sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["enemy"]["enemyRush"]) {
        data["playerDataDelta"]["deleted"]["sandboxPerm"]["SANDBOX_V2"]["sandbox_1"]["main"]["enemy"]["enemyRush"]
            .as_array_mut()
            .unwrap()
            .push(json!(rush));
    }
    for node in get_keys(&sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["node"]) {
        data["playerDataDelta"]["deleted"]["sandboxPerm"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["node"]
            .as_array_mut()
            .unwrap()
            .push(json!(node));
    }
    for food in get_keys(&sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["troop"]["food"]) {
        data["playerDataDelta"]["deleted"]["sandboxPerm"]["SANDBOX_V2"]["sandbox_1"]["troop"]["food"]
            .as_array_mut()
            .unwrap()
            .push(json!(food));
    }

    let json = json!({
        "template": {
            "SANDBOX_V2": {
                "sandbox_1": {
                    "status": {
                        "ver": 1,
                        "state": 0,
                        "ts": 0,
                        "isRift": false,
                        "isGuide": false,
                        "exploreMode": false,
                    },
                    "base": {
                        "baseLv": 0,
                        "upgradeProgress": [],
                        "trapLimit": {},
                        "portableUnlock": false,
                        "outpostUnlock": false,
                        "repairDiscount": 0,
                        "bossKill": [],
                    },
                    "main": {
                        "game": {
                            "mapId": "",
                            "day": 0,
                            "maxDay": 0,
                            "ap": 0,
                            "maxAp": 0,
                        },
                        "map": {
                            "season": {"type": 0, "remain": 0, "total": 0},
                            "zone": {},
                            "node": {},
                        },
                        "stage": {"node": {}},
                        "enemy": {"enemyRush": {}, "rareAnimal": {}},
                        "npc": {"node": {}, "favor": {}},
                        "report": {"settle": null, "daily": null},
                        "event": {"node": {}, "effect": []},
                    },
                    "rift": null,
                    "riftInfo": {
                        "isUnlocked": false,
                        "randomRemain": 0,
                        "difficultyLvMax": -1,
                        "teamLv": 0,
                        "fixFinish": [],
                        "reservation": null,
                        "gameInfo": null,
                        "settleInfo": null,
                    },
                    "quest": {"pending": [], "complete": []},
                    "mission": {"squad": []},
                    "troop": {
                        "food": {},
                        "squad": [
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                            {"slots": [], "tools": []},
                        ],
                        "usedChar": [],
                    },
                    "cook": {"drink": 0, "extraDrink": 0, "book": {}, "food": {}},
                    "build": {"book": {}, "building": {}, "tactical": {}, "animal": {}},
                    "bag": {"material": {}, "craft": []},
                    "tech": {"token": 6, "cent": 0, "unlock": []},
                    "bank": {"book": [], "coin": {}},
                    "buff": {"rune": {"global": [], "node": {}, "char": {}}},
                    "archive": {
                        "save": [],
                        "nextLoadTs": 0,
                        "loadTimes": 0,
                        "loadTs": 0,
                    },
                    "supply": {
                        "unlock": false,
                        "enable": false,
                        "slotCnt": 0,
                        "char": [],
                    },
                    "shop": {"unlock": false, "day": 0, "slots": []},
                    "month": {"rushPass": []},
                }
            }
        }
    });
    write_json(SANDBOX_JSON_PATH, json).unwrap_or(());

    Json(sandbox)
}

pub async fn eat_food(Json(payload): JSON) -> JSON {
    let mut sandbox = read_json(SANDBOX_JSON_PATH);
    let char_inst_id = payload["charInstId"].as_u64().unwrap();
    let food_inst_id = payload["foodInstId"].as_str().unwrap();

    let food = sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["cook"]["food"][food_inst_id].clone();
    sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["troop"]["food"][char_inst_id.to_string()] = json!({
        "id": food["id"],
        "sub": food["sub"],
        "day": 6
    });

    let buff = food["id"].as_str().unwrap();
    let sandbox_table = update_data(SANDBOX_TABLE_URL).await;

    let buff = if food["sub"].as_array().unwrap().contains(&json!("sandbox_1_condiment"))
        && get_keys(&sandbox_table["detail"]["SANDBOX_V2"]["sandbox_1"]["runeDatas"]).contains(&format!("{}_x", buff))
    {
        format!("{}_x", buff)
    } else {
        buff.to_string()
    };

    sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["char"][char_inst_id.to_string()] = json!([buff]);

    write_json(SANDBOX_JSON_PATH, &sandbox).unwrap_or(());

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}

pub async fn month_battle_start() -> JSON {
    Json(json!({
        "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
        "extraRunes": [],
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn month_battle_finish(Json(payload): JSON) -> JSON {
    let mut sandbox = read_json(SANDBOX_JSON_PATH);

    let node_id = "n12B9";

    if sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]
        .get("building")
        .is_none()
    {
        sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"] = json!([]);
    }

    for item in payload["sandboxV2Data"]["placedItems"].as_array().unwrap() {
        if item["value"].get("hpRatio").is_some() {
            sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "key": item["key"]["itemId"],
                    "pos": [item["key"]["position"]["row"], item["key"]["position"]["col"]],
                    "hpRatio": 10000,
                    "dir": item["value"]["direction"]
                }))
        } else {
            for bid in 0..sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                .as_array()
                .unwrap()
                .len()
            {
                let build = &sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"][bid];
                if build["pos"][0] == item["key"]["position"]["row"] && build["pos"][1] == item["key"]["position"]["col"] {
                    sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["main"]["stage"]["node"][node_id]["building"]
                        .as_array_mut()
                        .unwrap()
                        .remove(bid);
                    break;
                }
            }
        }
    }

    write_json(SANDBOX_JSON_PATH, &sandbox).unwrap_or(());

    Json(json!({
        "success": true,
        "firstPass": false,
        "enemyRushCount": [0, 0],
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}

pub async fn explore_mode(Json(payload): JSON) -> JSON {
    let explore_mode = payload["open"].as_bool().unwrap();
    let mut sandbox = read_json(SANDBOX_JSON_PATH);

    sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["status"]["exploreMode"] = json!(explore_mode);
    let mut explore_mode_buffs = vec![json!("normal_mode_buff1"), json!("normal_mode_buff3")];

    if explore_mode {
        sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["global"]
            .as_array_mut()
            .unwrap()
            .append(&mut explore_mode_buffs);
    } else {
        for buff in explore_mode_buffs {
            if let Value::String(buff) = buff {
                if sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["global"]
                    .as_array()
                    .unwrap()
                    .contains(&json!(buff))
                {
                    let v = sandbox["template"]["SANDBOX_V2"]["sandbox_1"]["buff"]["rune"]["global"]
                        .as_array_mut()
                        .unwrap();
                    let i = v.iter().position(|x| x == &json!(buff)).unwrap();
                    v.remove(i);
                }
            }
        }
    }

    write_json(SANDBOX_JSON_PATH, &sandbox).unwrap_or(());

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}
