use std::collections::HashMap;

use axum::Json;
use serde_json::json;

use crate::{
    constants::{tower::TOWERDATA_PATH, url::TOWER_TABLE_URL, user::USER_JSON_PATH},
    core::time,
    utils::{
        enumerate,
        game::{decrypt_battle_data, update_data},
        json::{get_keys, get_length, read_json, write_json, JSON},
        random::sample,
        str,
    },
};

fn current_coord(id: &str) -> usize {
    let tower = read_json(TOWERDATA_PATH);

    for (index, layer) in enumerate(tower["tower"]["current"]["layer"].as_array().unwrap()) {
        if layer["id"].as_str().unwrap() == id {
            return index;
        }
    }

    0
}

fn create_recruit_list() {
    let mut tower = read_json(TOWERDATA_PATH);
    let user = read_json(USER_JSON_PATH);
    let mut candidates = Vec::new();
    let mut all_cards = Vec::new();
    let mut inst_map = HashMap::new();
    for key in get_keys(&user["user"]["troop"]["chars"]) {
        let inst = user["user"]["troop"]["chars"][&key]["instId"].as_u64().unwrap().to_string();
        inst_map.insert(inst.clone(), key);
        all_cards.push(inst);
    }
    let mut used_cards = Vec::new();
    for key in get_keys(&tower["tower"]["current"]["cards"]) {
        used_cards.push(user["tower"]["current"]["cards"][&key]["relation"].as_str().unwrap().to_string());
    }
    let mut picked_cards = Vec::new();
    for card in all_cards {
        if !used_cards.contains(&card) {
            picked_cards.push(card);
        }
    }
    let cards = sample(picked_cards, 5);

    for card in cards {
        let card = inst_map[&card].clone();
        let char = user["user"]["troop"]["chars"][&card].clone();
        candidates.push(json!({
            "groupId": char["charId"],
            "type": "CHAR",
            "cards": [
                {
                    "instId": "0",
                    "type": "CHAR",
                    "charId": char["charId"],
                    "relation": card,
                    "evolvePhase": char["evolvePhase"],
                    "level": char["level"],
                    "favorPoint": char["favorPoint"],
                    "potentialRank": char["potentialRank"],
                    "mainSkillLvl": char["mainSkillLvl"],
                    "skills": char["skills"],
                    "defaultSkillIndex": char["defaultSkillIndex"],
                    "currentEquip": char["currentEquip"],
                    "equip": char["equip"],
                    "skin": char["skin"],
                }
            ],
        }))
    }

    tower["tower"]["current"]["halftime"]["candidate"] = json!(candidates);
    write_json(TOWERDATA_PATH, tower);
}

pub async fn tower_create_game(Json(payload): JSON) -> JSON {
    let tower_table = update_data(TOWER_TABLE_URL).await;
    let is_hard = payload["isHard"].as_i64().unwrap();
    let tower_id = payload["tower"].as_str().unwrap();

    let (tower, hard_mode) = if is_hard == 1 {
        (tower_table["towers"][tower_id]["hardLevels"].as_array().unwrap(), true)
    } else {
        (tower_table["towers"][tower_id]["levels"].as_array().unwrap(), false)
    };

    let mut lvls = Vec::new();

    for level in tower {
        let level = level.as_str().unwrap();
        lvls.push(json!({
            "id": level,
            "try": 0,
            "pass": false,
        }))
    }

    let tower_data = json!({
        "tower": {
            "current": {
                "cards": {},
                "godCard": {
                    "id": "",
                    "subGodCardId": "",
                },
                "halftime": {
                    "canGiveUp": false,
                    "candidate": [],
                    "count": 0,
                },
                "layer": lvls,
                "reward": {
                    "high": 0,
                    "low": 0,
                },
                "status": {
                    "coord": 0,
                    "isHard": hard_mode,
                    "start": time(),
                    "state": "INIT_GOD_CARD",
                    "strategy": "OPTIMIZE",
                    "tactical": {
                        "CASTER": "",
                        "MEDIC": "",
                        "PIONEER": "",
                        "SNIPER": "",
                        "SPECIAL": "",
                        "SUPPORT": "",
                        "TANK": "",
                        "WARRIOR": "",
                    },
                    "tower": tower_id,
                },
                "trap": [],
            }
        },
        "currentStage": "",
    });

    write_json(TOWERDATA_PATH, &tower_data);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "tower": tower_data["tower"]
            },
            "deleted": {},
        }
    }))
}

pub async fn tower_init_godcard(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);

    tower["tower"]["current"]["status"]["state"] = json!("INIT_BUFF");
    tower["tower"]["current"]["godCard"]["id"] = json!(payload["godCardId"]);

    write_json(TOWERDATA_PATH, &tower);
    Json(json!({
        "playerDataDelta": {
            "modified": {"tower": tower["tower"]},
            "deleted": {},
        }
    }))
}

pub async fn tower_init_game(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);
    tower["tower"]["current"]["status"]["state"] = json!("INIT_CARD");
    tower["tower"]["current"]["status"]["strategy"] = json!(payload["strategy"]);
    tower["tower"]["current"]["status"]["tactical"] = json!(payload["tactical"]);
    write_json(TOWERDATA_PATH, &tower);
    Json(json!({
        "playerDataDelta": {
            "modified": {"tower": tower["tower"]},
            "deleted": {},
        }
    }))
}

pub async fn tower_init_card(Json(payload): JSON) -> JSON {
    let mut cnt = 1;
    let mut tower = read_json(TOWERDATA_PATH);
    let user = read_json(USER_JSON_PATH);

    tower["tower"]["current"]["status"]["state"] = json!("STANDBY");

    for slot in payload["slots"].as_array().unwrap() {
        tower["tower"]["current"]["cards"][cnt.to_string()] = json!({
            "charId": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["charId"],
            "currentEquip": slot["currentEquip"],
            "defaultEquip": slot["skillIndex"],
            "equip": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["equip"],
            "evolvePhase": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["evolvePhase"],
            "favorPoint": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["favorPoint"],
            "instId": cnt.to_string(),
            "level": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["level"],
            "mainSkillLvl": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["mainSkillLvl"],
            "potentialRank": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["potentialRank"],
            "relation": str(&slot["charInstId"]),
            "skills": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["skills"],
            "skin": user["user"]["troop"]["chars"][str(&slot["charInstId"])]["skin"],
            "type": "CHAR",
        });
        cnt += 1;
    }

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "tower": tower["tower"]
            },
            "deleted": {},
        }
    }))
}

pub async fn tower_battle_start(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);
    let stage_id = payload["stageId"].as_str().unwrap();

    tower["tower"]["current"]["status"]["coord"] = json!(current_coord(stage_id));
    tower["currentStage"] = json!(stage_id);

    for stage in tower["tower"]["current"]["layer"].as_array_mut().unwrap() {
        if stage["id"].as_str().unwrap() == stage_id {
            stage["try"] = json!(stage["try"].as_i64().unwrap() + 1);
        }
    }

    write_json(TOWERDATA_PATH, &tower);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "tower": tower["tower"]
            },
            "deleted": {},
        }
    }))
}

pub async fn tower_battle_finish(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);
    let battle_data = decrypt_battle_data(payload["data"].as_str().unwrap(), None);
    let mut trap = Vec::new();

    let len = tower["tower"]["current"]["layer"].as_array().unwrap().len();

    let data = if battle_data["completeState"].as_i64().unwrap() == 1 {
        let coord = tower["tower"]["current"]["status"]["coord"].as_i64().unwrap();
        let coord = { coord - 1 } as usize;
        let i = tower["tower"]["current"]["layer"][coord]["try"].as_u64().unwrap();
        tower["tower"]["current"]["layer"][coord]["try"] = json!(i + 1);
        json!({
            "drop": [],
            "isNewRecord": false,
            "trap": [],
            "playerDataDelta": {"modified": {}, "deleted": {}},
        })
    } else {
        if tower["currentStage"].as_str().unwrap() == tower["tower"]["current"]["layer"][2]["id"] {
            tower["tower"]["current"]["status"]["state"] = json!("SUB_GOD_CARD_RECRUIT");

            for battle_info in get_keys(&battle_data["battleData"]["stats"]["extraBattleInfo"]) {
                if battle_info.starts_with("DETAILED") && battle_info.ends_with("legion_gain_reward_trap") {
                    let infos = battle_info.split(',').collect::<Vec<&str>>();
                    trap.push(json!({
                        "id": infos[1],
                        "alias": infos[2],
                    }));
                }
            }

            tower["tower"]["current"]["trap"] = json!(trap);
        } else if tower["currentStage"].as_str().unwrap() == tower["tower"]["current"]["layer"][len - 1]["id"].as_str().unwrap() {
            tower["tower"]["current"]["status"]["state"] = json!("END");
        } else {
            tower["tower"]["current"]["status"]["state"] = json!("RECRUIT");
        }

        let cur_stage = tower["currentStage"].as_str().unwrap();
        let mut tower = read_json(TOWERDATA_PATH);

        for stage in tower["tower"]["current"]["layer"].as_array_mut().unwrap() {
            if stage["id"].as_str().unwrap() == cur_stage {
                stage["try"] = json!(stage["try"].as_i64().unwrap() + 1);
            }
        }

        let coord = tower["tower"]["current"]["status"]["coord"].as_u64().unwrap();
        let cnt = tower["tower"]["current"]["halftime"]["count"].as_u64().unwrap();

        tower["tower"]["current"]["status"]["coord"] = json!(coord + 1);
        tower["tower"]["current"]["halftime"]["count"] = json!(cnt + 1);

        write_json(TOWERDATA_PATH, &tower);
        create_recruit_list();
        let tower = read_json(TOWERDATA_PATH);
        json!({
            "drop": [],
            "isNewRecord": false,
            "trap": trap,
            "playerDataDelta": {
                "modified": {
                    "tower": tower["tower"]
                },
                "deleted": {},
            },
        })
    };
    Json(data)
}

pub async fn tower_recruit(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);
    let user_data = read_json(USER_JSON_PATH);

    if tower["tower"]["current"]["halftime"]["count"].as_i64().unwrap() == 1 {
        tower["tower"]["current"]["status"]["state"] = json!("RECRUIT");
        tower["tower"]["current"]["halftime"]["count"] = json!(0);
    } else {
        tower["tower"]["current"]["status"]["state"] = json!("STANDBY");
    }

    if payload["giveUp"].as_i64().unwrap() != 1 {
        let cnt = get_length(&tower["tower"]["current"]["cards"]) + 2;
        let char_id = payload["charId"].as_str().unwrap();
        let char_inst_id = user_data["user"]["dexNav"]["character"][char_id]["charInstId"]
            .as_u64()
            .unwrap()
            .to_string();
        tower["tower"]["current"]["cards"][cnt.to_string()] = json!({
            "charId": char_id,
            "currentEquip": user_data["user"]["troop"]["chars"][&char_inst_id]["currentEquip"],
            "defaultSkillIndex": user_data["user"]["troop"]["chars"][&char_inst_id]["defaultSkillIndex"],
            "equip": user_data["user"]["troop"]["chars"][&char_inst_id]["equip"],
            "evolvePhase": user_data["user"]["troop"]["chars"][&char_inst_id]["evolvePhase"],
            "favorPoint": user_data["user"]["troop"]["chars"][&char_inst_id]["favorPoint"],
            "instId": cnt.to_string(),
            "level": user_data["user"]["troop"]["chars"][&char_inst_id]["level"],
            "mainSkillLvl": user_data["user"]["troop"]["chars"][&char_inst_id]["mainSkillLvl"],
            "potentialRank": user_data["user"]["troop"]["chars"][&char_inst_id]["potentialRank"],
            "skills": user_data["user"]["troop"]["chars"][&char_inst_id]["skills"],
            "skin": user_data["user"]["troop"]["chars"][&char_inst_id]["skin"],
            "type": "CHAR",
            "relation": char_inst_id
        });
    }
    write_json(TOWERDATA_PATH, &tower);
    create_recruit_list();
    let tower = read_json(TOWERDATA_PATH);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "tower": tower["tower"]
            },
            "deleted": {},
        }
    }))
}

pub async fn tower_choose_sub_godcard(Json(payload): JSON) -> JSON {
    let mut tower = read_json(TOWERDATA_PATH);

    tower["tower"]["current"]["status"]["state"] = json!("STANDBY");
    tower["tower"]["current"]["godCard"]["subGodCardId"] = json!(payload["subGodCardId"]);
    write_json(TOWERDATA_PATH, &tower);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "tower": tower["tower"]
            },
            "deleted": {},
        }
    }))
}

pub async fn tower_settle_game() -> JSON {
    let tower = read_json(TOWERDATA_PATH);

    let keys = get_keys(&tower["tower"]["current"]["cards"]);

    Json(json!({
        "reward": {
            "high": {
                "cnt": 0,
                "from": 24,
                "to": 24
            },
            "low": {
                "cnt": 0,
                "from": 60,
                "to": 60
            },
        },
        "ts": time(),
        "playerDataDelta": {
            "modified": {
                "tower": {
                    "current": {
                        "status": {
                            "state": "NONE",
                            "tower": "",
                            "coord": 0,
                            "tactical": {
                                "PIONEER": "",
                                "WARRIOR": "",
                                "TANK": "",
                                "SNIPER": "",
                                "CASTER": "",
                                "SUPPORT": "",
                                "MEDIC": "",
                                "SPECIAL": "",
                            },
                            "startegy": "OPTIMIZE",
                            "start": 0,
                            "isHard": false,
                        },
                        "layer": [],
                        "cards": {},
                        "godCard": {
                            "id": "",
                            "subGodCardId": "",
                        },
                        "halftime": {"count": 0, "candidate": [], "canGiveUp": false},
                        "trap": [],
                        "raward": {},
                    }
                }
            },
            "deleted": {
                "tower": {
                    "current": {
                        "cards": keys
                    }
                }
            },
        },
    }))
}
