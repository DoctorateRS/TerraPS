pub mod quest {
    use axum::Json;
    use serde_json::json;

    use crate::{
        constants::user::USER_JSON_PATH,
        utils::json::{get_keys, read_json, write_json, JSON},
    };

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

    pub async fn _relic_select(Json(payload): JSON) -> JSON {
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

pub mod april_fools {
    // TODO: Implement April Fools
}
