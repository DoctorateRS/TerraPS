pub mod quest {
    use axum::Json;
    use serde_json::json;

    use crate::{
        constants::user::{BATTLE_REPLAY_JSON_PATH, USER_JSON_PATH},
        utils::json::{get_keys, read_json, write_json, JSON},
    };

    pub async fn quest_battle_start(Json(payload): JSON) -> JSON {
        let stage_id = payload["stageId"].as_str().unwrap();
        let mut replay_data = read_json(BATTLE_REPLAY_JSON_PATH);
        replay_data["current"] = json!(stage_id);
        write_json(BATTLE_REPLAY_JSON_PATH, replay_data);
        Json(json!({
            "apFailReturn": 0,
            "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
            "inApProtectPeriod": false,
            "isApProtect": 0,
            "notifyPowerScoreNotEnoughIfFailed": false,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            },
            "result": 0
        }))
    }

    pub async fn quest_battle_finish() -> JSON {
        Json(json!({
            "result":0,
            "apFailReturn": 0,
            "expScale": 1.2,
            "goldScale": 1.2,
            "rewards": [],
            "firstRewards": [],
            "unlockStages": [],
            "unusualRewards": [],
            "additionalRewards": [],
            "furnitureRewards": [],
            "overrideRewards": [],
            "alert": [],
            "suggestFriend": false,
            "pryResult": [],
            "itemReturn": [],
            "wave": 0,
            "milestoneBefore": 0,
            "milestoneAdd": 0,
            "isMileStoneMax": false,
            "tokenAdd": 0,
            "isTokenMax": false,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn quest_save_battle_replay() {}

    pub async fn quest_get_battle_replay() {}

    pub async fn squad_change_name(Json(payload): JSON) -> JSON {
        let mut data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "squads":{}
                    }
                },
                "deleted":{}
            }
        });

        if payload.get("squadId").is_some() && payload.get("name").is_some() {
            let squad_id = payload["squadId"].as_u64().unwrap().to_string();
            let name = payload["name"].clone();
            data["playerDataDelta"]["modified"]["troop"]["squads"][&squad_id]["name"] = name.clone();
            let mut user_data = read_json(USER_JSON_PATH);
            user_data["user"]["troop"]["squads"][&squad_id]["name"] = name;
            write_json(USER_JSON_PATH, user_data);
        }

        Json(data)
    }

    pub async fn squad_set_formation(Json(payload): JSON) -> JSON {
        let mut data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "squads":{}
                    }
                },
                "deleted":{}
            }
        });

        if payload.get("squadId").is_some() && payload.get("slots").is_some() {
            let squad_id = payload["squadId"].as_u64().unwrap().to_string();
            let slots = payload["slots"].clone();
            data["playerDataDelta"]["modified"]["troop"]["squads"][&squad_id]["slots"] = slots.clone();
            let mut user_data = read_json(USER_JSON_PATH);
            user_data["user"]["troop"]["squads"][&squad_id]["slots"] = slots;
            write_json(USER_JSON_PATH, user_data);
        }

        Json(data)
    }

    pub async fn confirm_battle_car(Json(payload): JSON) -> JSON {
        let car = payload["car"].clone();
        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "car": {
                        "battleCar": car
                    }
                },
                "deleted": {}
            }
        }))
    }

    pub async fn act_20_competition_start() -> JSON {
        Json(json!({
            "result": 0,
            "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn act_20_competition_finish() -> JSON {
        Json(json!({
            "performance": 0,
            "expression": 0,
            "operation": 0,
            "total": 0,
            "level": "B",
            "isNew": false,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn quest_battle_continue() -> JSON {
        Json(json!({
            "result": 0,
            "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
            "apFailReturn": 0,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn set_tool(Json(payload): JSON) -> JSON {
        let mut tools = json!({
            "tool_trap": 1,
            "tool_wirebug": 1,
            "tool_flashbomb": 1,
            "tool_bomb": 1
        });

        for tool in get_keys(&payload["tools"]) {
            tools[tool] = json!(2);
        }

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "activity": {
                        "TYPE_ACT24SIDE": {
                            "act24side": {
                                "tool": tools
                            }
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }

    pub async fn set_trap_squad(Json(payload): JSON) -> JSON {
        let trap_domain_id = payload["trapDomainId"].as_str().unwrap();
        let trap_squad = payload["trapSquad"].clone();

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "templateTrap": {
                        "domains": {
                            trap_domain_id: {
                                "squad": trap_squad
                            }
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }
}

pub mod bossrush {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

    pub async fn relic_select(Json(payload): JSON) -> JSON {
        let activity_id = payload["activityId"].as_str().unwrap();
        let relic_id = payload["relicId"].clone();

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "activity": {
                        "BOSS_RUSH": {
                            activity_id: {
                                "relic": {
                                    "select": relic_id
                                }
                            }
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }
}

pub mod story_review {
    use crate::utils::json::JSON;
    use axum::Json;
    use serde_json::json;

    pub async fn mark_story_acce_known() -> JSON {
        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "storyreview": {
                        "tags": {
                            "knownStoryAcceleration": 1
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }

    pub async fn read_story() -> JSON {
        Json(json!({
            "readCount": 1,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }
}
pub mod april_fools {
    use axum::Json;
    use serde_json::json;

    use crate::utils::{
        battle_data::BattleDataDecoder,
        enumerate,
        json::{get_keys, JSON},
    };

    pub async fn act5_fun_battle_finish(Json(payload): JSON) -> JSON {
        let decoder = BattleDataDecoder::new();
        let battle_data = decoder.decrypt_battle_data(payload["data"].as_str().unwrap().to_string()).unwrap();
        let mut score = 0;
        for data in get_keys(&battle_data["battleData"]["stats"]["extraBattleInfo"]) {
            if data.starts_with("SIMPLE,money,") {
                score = battle_data["battleData"]["stats"]["extraBattleInfo"][data]
                    .as_str()
                    .unwrap()
                    .split(',')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }
        Json(json!({
            "result": 0,
            "score": score,
            "isHighScore": false,
            "npcResult": {},
            "playerResult": {
                "totalWin": 0,
                "streak": 0,
                "totalRound": 10
            },
            "reward": [],
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn act4_fun_battle_finish() -> JSON {
        let mut mat_vec = Vec::new();
        for (id, mat_id) in enumerate(vec![
            "spLiveMat_tr_1",
            "spLiveMat_tr_2",
            "spLiveMat_01_1",
            "spLiveMat_01_2",
            "spLiveMat_01_3",
            "spLiveMat_01_4",
            "spLiveMat_01_5",
            "spLiveMat_01_6",
            "spLiveMat_01_7",
            "spLiveMat_01_8",
            "spLiveMat_01_9",
            "spLiveMat_02_1",
            "spLiveMat_02_2",
            "spLiveMat_02_3",
            "spLiveMat_02_4",
            "spLiveMat_02_5",
            "spLiveMat_02_6",
            "spLiveMat_02_7",
            "spLiveMat_02_8",
            "spLiveMat_02_9",
            "spLiveMat_03_1",
            "spLiveMat_03_2",
            "spLiveMat_03_3",
            "spLiveMat_03_4",
            "spLiveMat_03_5",
            "spLiveMat_03_6",
            "spLiveMat_03_7",
            "spLiveMat_03_8",
            "spLiveMat_03_9",
        ]) {
            mat_vec.push(json!({
                "instId": id,
                "materialId": mat_id,
                "materialType": 1
            }));
        }
        Json(json!({
            "materials": mat_vec,
            "liveId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn act4_fun_live_settle() -> JSON {
        Json(json!({
            "ending": "goodending_1",
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }
}
