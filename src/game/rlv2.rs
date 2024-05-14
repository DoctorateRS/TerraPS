use axum::Json;
use rand::Rng;
use serde_json::{json, Value};

use crate::{
    constants::{config::CONFIG_JSON_PATH, user::RLV2_JSON_PATH},
    utils::{
        battle_data::FallbackMode,
        enumerate,
        game::decrypt_battle_data,
        json::{get_keys, read_json, write_json, JSON},
        random::TRng,
        rlv2::*,
    },
};

pub async fn rlv2_give_up_game() -> JSON {
    Json(json!({
            "result": "ok",
            "playerDataDelta": {
                "modified": {
                    "rlv2": {
                        "current": {
                            "player": Value::Null,
                            "record": Value::Null,
                            "map": Value::Null,
                            "troop": Value::Null,
                            "inventory": Value::Null,
                            "game": Value::Null,
                            "buff": Value::Null,
                            "module": Value::Null
                        }
                    }
                },
                "deleted": {}
            }
        }
    ))
}

pub async fn rlv2_create_game(Json(payload): JSON) -> JSON {
    let theme = payload["theme"].as_str().unwrap();
    let mode = payload["mode"].as_str().unwrap();

    let mode = if mode == "MONTH_TEAM" || mode == "CHALLENGE" {
        "NORMAL"
    } else {
        mode
    };

    let mode_grade = payload["modeGrade"].clone();

    let (bands, ending) = match theme {
        "rogue_1" => (
            vec![
                "rogue_1_band_1",
                "rogue_1_band_2",
                "rogue_1_band_3",
                "rogue_1_band_4",
                "rogue_1_band_5",
                "rogue_1_band_6",
                "rogue_1_band_7",
                "rogue_1_band_8",
                "rogue_1_band_9",
                "rogue_1_band_10",
            ],
            "ro_ending_1",
        ),
        "rogue_2" => (
            vec![
                "rogue_2_band_1",
                "rogue_2_band_2",
                "rogue_2_band_3",
                "rogue_2_band_4",
                "rogue_2_band_5",
                "rogue_2_band_6",
                "rogue_2_band_7",
                "rogue_2_band_8",
                "rogue_2_band_9",
                "rogue_2_band_10",
                "rogue_2_band_11",
                "rogue_2_band_12",
                "rogue_2_band_13",
                "rogue_2_band_14",
                "rogue_2_band_15",
                "rogue_2_band_16",
                "rogue_2_band_17",
                "rogue_2_band_18",
                "rogue_2_band_19",
                "rogue_2_band_20",
                "rogue_2_band_21",
                "rogue_2_band_22",
            ],
            "ro2_ending_1",
        ),
        "rogue_3" => (
            vec![
                "rogue_3_band_1",
                "rogue_3_band_2",
                "rogue_3_band_3",
                "rogue_3_band_4",
                "rogue_3_band_5",
                "rogue_3_band_6",
                "rogue_3_band_7",
                "rogue_3_band_8",
                "rogue_3_band_9",
                "rogue_3_band_10",
                "rogue_3_band_11",
                "rogue_3_band_12",
                "rogue_3_band_13",
            ],
            "ro3_ending_1",
        ),
        _ => (vec![], ""),
    };

    let mut rlv2 = json!({
        "player": {
            "state": "INIT",
            "property": {
                "exp": 0,
                "level": 10,
                "maxLevel": 10,
                "hp": {"current": 10000, "max": 10000},
                "gold": 600,
                "shield": 10000,
                "capacity": 10000,
                "population": {"cost": 0, "max": 6},
                "conPerfectBattle": 0,
            },
            "cursor": {"zone": 0, "position": Value::Null},
            "trace": [],
            "pending": [
                {
                    "index": "e_0",
                    "type": "GAME_INIT_RELIC",
                    "content": {
                        "initRelic": {
                            "step": [1, 3],
                            "items": {},
                        }
                    },
                },
                {
                    "index": "e_1",
                    "type": "GAME_INIT_RECRUIT_SET",
                    "content": {
                        "initRecruitSet": {
                            "step": [2, 3],
                            "option": ["recruit_group_1"],
                        }
                    },
                },
                {
                    "index": "e_2",
                    "type": "GAME_INIT_RECRUIT",
                    "content": {
                        "initRecruit": {
                            "step": [3, 3],
                            "tickets": [],
                            "showChar": [],
                            "team": Value::Null,
                        }
                    },
                },
            ],
            "status": {"bankPut": 0},
            "toEnding": ending,
            "chgEnding": false,
        },
        "record": {"brief": Value::Null},
        "map": {"zones": {}},
        "troop": {
            "chars": {},
            "expedition": [],
            "expeditionReturn": Value::Null,
            "hasExpeditionReturn": false,
        },
        "inventory": {
            "relic": {},
            "recruit": {},
            "trap": Value::Null,
            "consumable": {},
            "exploreTool": {},
        },
        "game": {
            "mode": mode,
            "predefined": Value::Null,
            "theme": theme,
            "outer": {"support": false},
            "start": 1695000000,
            "modeGrade": mode_grade,
            "equivalentGrade": mode_grade,
        },
        "buff": {"tmpHP": 0, "capsule": Value::Null, "squadBuff": []},
        "module": {},
    });

    for (id, band) in enumerate(bands) {
        rlv2["player"]["pending"][0]["content"]["initRelic"]["items"][id.to_string()] = json!({"id": band, "count": 1});
    }

    write_json(RLV2_JSON_PATH, &rlv2);

    let config = read_json(CONFIG_JSON_PATH);

    if config["rlv2Config"]["allChars"].as_bool().unwrap() {
        let ticket = match theme {
            "rogue_1" => "rogue_1_recruit_ticket_all",
            "rogue_2" => "rogue_2_recruit_ticket_all",
            "rogue_3" => "rogue_3_recruit_ticket_all",
            _ => "",
        };
        let chars = get_chars(true);
        for (id, mut char) in enumerate(chars) {
            let tkt_id = format!("t_{}", id);
            let chr_id = (id + 1).to_string();
            char["instId"] = json!(&chr_id);
            rlv2["inventory"]["recruit"][&tkt_id] = json!({
                "index": tkt_id,
                "id": ticket,
                "state": 2,
                "list": [],
                "result": char,
                "ts": 1695000000,
                "from": "initial",
                "mustExtra": 0,
                "needAssist": true,
            });
            rlv2["troop"]["chars"][&chr_id] = char;
        }
    }

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_choose_init_relic(Json(payload): JSON) -> JSON {
    let select = payload["select"].as_str().unwrap();
    let rlv2_static = read_json(RLV2_JSON_PATH);
    let mut rlv2 = read_json(RLV2_JSON_PATH);

    let band = rlv2_static["player"]["pending"].as_array().unwrap();
    let vec = band[1..{ band.len() - 1 }].to_vec();
    let band = band[0]["content"]["initRelic"]["items"][select]["id"].as_str().unwrap();
    rlv2["player"]["pending"] = json!(vec);

    rlv2["inventory"]["relic"]["r_0"] = json!({
        "index": "r_0",
        "id": band,
        "count": 1,
        "ts": 1695000000,
    });

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_choice_select() -> JSON {
    let rlv2 = read_json(RLV2_JSON_PATH);
    write_json(RLV2_JSON_PATH, &rlv2);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_choose_init_recruit_set() -> JSON {
    let rlv2_static = read_json(RLV2_JSON_PATH);
    let mut rlv2 = read_json(RLV2_JSON_PATH);

    let vec = rlv2_static["player"]["pending"].as_array().unwrap();
    let vec = if !vec.is_empty() {
        vec[1..{ vec.len() - 1 }].to_vec()
    } else {
        vec![]
    };
    rlv2["player"]["pending"] = json!(vec);

    let config = read_json(CONFIG_JSON_PATH);

    if !config["rlv2Config"]["allChars"].as_bool().unwrap() {
        let mut tkts = vec![];
        for _ in 0..3 {
            let ticket_id = get_next_tkt(&rlv2);
            add_ticket(&mut rlv2, &ticket_id);
            tkts.push(ticket_id);
        }
        rlv2["player"]["pending"][0]["content"]["initRecruit"]["tickets"] = json!(tkts);
    }

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_activate_recruit_tkt(Json(payload): JSON) -> JSON {
    let tkt_id = payload["id"].as_str().unwrap();
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    activate_tkt(&mut rlv2, tkt_id);
    write_json(RLV2_JSON_PATH, &rlv2);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_recruit_char(Json(payload): JSON) -> JSON {
    let tkt_id = payload["ticketIndex"].as_str().unwrap();
    let opt_id = payload["optionId"].as_str().unwrap().parse::<usize>().unwrap();
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["pending"].as_array_mut().unwrap().remove(0);

    let char_id = get_next_char_id(&rlv2);
    let mut char = rlv2["inventory"]["recruit"][tkt_id]["list"][opt_id].clone();
    char["instId"] = json!(char_id);
    rlv2["inventory"]["recruit"][tkt_id]["state"] = json!(2);
    rlv2["inventory"]["recruit"][tkt_id]["list"] = json!([]);
    rlv2["inventory"]["recruit"][tkt_id]["result"] = json!(&char);
    rlv2["troop"]["chars"][char_id] = json!(&char);

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "chars": [char],
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        },
    }))
}

pub async fn rlv2_close_tkt(Json(payload): JSON) -> JSON {
    let tkt_id = payload["id"].as_str().unwrap();
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["pending"].as_array_mut().unwrap().pop();
    rlv2["inventory"]["recruit"][tkt_id]["state"] = json!(2);
    rlv2["inventory"]["recruit"][tkt_id]["list"] = json!([]);
    rlv2["inventory"]["recruit"][tkt_id]["result"] = json!(Value::Null);

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_finish_event() -> JSON {
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("WAIT_MOVE");
    rlv2["player"]["cursor"]["zone"] = json!(1);
    rlv2["player"]["pending"].as_array_mut().unwrap().clear();
    write_json(RLV2_JSON_PATH, &rlv2);
    let theme = rlv2["game"]["theme"].as_str().unwrap();

    rlv2["map"]["zones"] = get_map(theme).await;

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

async fn mv_n_battle_start(payload: Value) -> Value {
    let mut rng = TRng::new();
    let mut hf = vec![];
    let stage_id = payload["stageId"].as_str().unwrap();
    let x = payload["to"]["x"].as_u64().unwrap();
    let y = payload["to"]["y"].as_u64().unwrap();

    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("PENDING");
    rlv2["player"]["cursor"]["position"] = json!({"x": x, "y": y});
    let trace_loc = rlv2["player"]["cursor"].clone();
    let trace = match rlv2["player"]["trace"].as_array_mut() {
        Some(trace) => trace,
        None => &mut hf,
    };
    trace.push(trace_loc);
    rlv2["player"]["trace"] = json!(trace);

    let p_index = get_next_pending(&rlv2);
    let mut buffs = get_buffs(&rlv2, stage_id).await;
    let theme = rlv2["game"]["theme"].as_str().unwrap();

    let box_info = match theme {
        "rogue_1" => json!({}),
        "rogue_2" => {
            let trap = ["trap_065_normbox", "trap_066_rarebox", "trap_068_badbox"][rng.0.gen_range(0..3) as usize];
            json!({trap: 100})
        }
        "rogue_3" => {
            let trap = ["trap_108_smbox", "trap_109_smrbox", "trap_110_smbbox"][rng.0.gen_range(0..3) as usize];
            json!({trap: 100})
        }
        _ => json!({}),
    };

    let mut dice_roll = vec![];

    if theme == "rogue_2" {
        let mut dice_upgrade = 0;
        let band = rlv2["inventory"]["relic"]["r_0"]["id"].as_str().unwrap();
        if band == "rogue_2_band_16" || band == "rogue_2_band_17" || band == "rogue_2_band_18" {
            dice_upgrade += 1;
        }
        for relic in get_keys(&rlv2["inventory"]["relic"]) {
            if rlv2["inventory"]["relic"][&relic]["id"].as_str().unwrap() == "rogue_2_relic_1" {
                dice_upgrade += 1;
            }
        }
        let (dice_fcnt, dice_id) = match dice_upgrade {
            0 => (6, "trap_067_dice1"),
            1 => (8, "trap_088_dice2"),
            _ => (12, "trap_089_dice3"),
        };
        for _ in 0..100 {
            dice_roll.push(rng.0.gen_range(1_usize..dice_fcnt + 1));
        }
        buffs.push(json!({
            "key": "misc_insert_token_card",
            "blackboard": [
                {"key": "token_key", "value": 0, "valueStr": dice_id},
                {"key": "level", "value": 1, "valueStr": Value::Null},
                {"key": "skill", "value": 0, "valueStr": Value::Null},
                {"key": "cnt", "value": 100, "valueStr": Value::Null},
            ],
        }));
    }

    rlv2["player"]["pending"].as_array_mut().unwrap_or(&mut hf).insert(
        0,
        json!({
            "index": p_index,
            "type": "BATTLE",
            "content": {
                "battle": {
                    "state": 1,
                    "chestCnt": 100,
                    "goldTrapCnt": 100,
                    "diceRoll": dice_roll,
                    "boxInfo": box_info,
                    "tmpChar": [],
                    "sanity": 0,
                    "unKeepBuff": buffs,
                }
            },
        }),
    );

    if rlv2["player"]["pending"].is_array() {
        rlv2["player"]["pending"] = json!(hf);
    };

    write_json(RLV2_JSON_PATH, &rlv2);
    rlv2
}

pub async fn rlv2_mv_n_battle_start(Json(payload): JSON) -> JSON {
    let rlv2 = mv_n_battle_start(payload).await;

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_battle_finish(Json(payload): JSON) -> JSON {
    let battle_data = decrypt_battle_data(payload["data"].as_str().unwrap(), None, FallbackMode::Rlv2);

    let mut rlv2 = read_json(RLV2_JSON_PATH);
    if battle_data["completeState"].as_i64().unwrap() != 1 {
        rlv2["player"]["pending"].as_array_mut().unwrap().remove(0);
        let theme = rlv2["game"]["theme"].as_str().unwrap();
        let tkt = match theme {
            "rogue_1" => "rogue_1_recruit_ticket_all",
            "rogue_2" => "rogue_2_recruit_ticket_all",
            "rogue_3" => "rogue_3_recruit_ticket_all",
            _ => "",
        };
        let pending_index = get_next_pending(&rlv2);
        rlv2["player"]["pending"].as_array_mut().unwrap().insert(
            0,
            json!({
                "index": pending_index,
                "type": "BATTLE_REWARD",
                "content": {
                    "battleReward": {
                        "earn": {
                            "damage": 0,
                            "hp": 0,
                            "shield": 0,
                            "exp": 0,
                            "populationMax": 0,
                            "squadCapacity": 0,
                            "maxHpUp": 0,
                        },
                        "rewards": [
                            {
                                "index": 0,
                                "items": [{"sub": 0, "id": tkt, "count": 1}],
                                "done": 0,
                            }
                        ],
                        "show": "1",
                    }
                },
            }),
        )
    } else {
        rlv2["player"]["state"] = json!("WAIT_MOVE");
        rlv2["player"]["pending"].as_array_mut().unwrap().clear();
        rlv2["player"]["cursor"]["position"]["x"] = json!(0);
        rlv2["player"]["cursor"]["position"]["y"] = json!(0);
        rlv2["player"]["trace"].as_array_mut().unwrap().pop();
    }

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_finish_battle_reward() -> JSON {
    let mut rlv2 = read_json(RLV2_JSON_PATH);

    rlv2["player"]["state"] = json!("WAIT_MOVE");
    rlv2["player"]["pending"].as_array_mut().unwrap().clear();
    rlv2["player"]["cursor"]["position"]["x"] = json!(0);
    rlv2["player"]["cursor"]["position"]["y"] = json!(0);
    rlv2["player"]["trace"].as_array_mut().unwrap().pop();

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_mv_to(Json(payload): JSON) -> JSON {
    let x = payload["to"]["x"].clone();
    let y = payload["to"]["y"].clone();

    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("PENDING");
    rlv2["player"]["cursor"]["position"] = json!({"x": x, "y": y});
    let theme = rlv2["game"]["theme"].as_str().unwrap();
    let goods = get_goods(theme).await;
    let cursor = rlv2["player"]["cursor"].clone();
    rlv2["player"]["trace"].as_array_mut().unwrap().push(cursor);
    let pending_index = get_next_pending(&rlv2);
    rlv2["player"]["pending"].as_array_mut().unwrap().insert(
        0,
        json!({
            "index": pending_index,
            "type": "SHOP",
            "content": {
                "shop": {
                    "bank": {
                        "open": false,
                        "canPut": false,
                        "canWithdraw": false,
                        "withdraw": 0,
                        "cost": 1,
                    },
                    "id": "just_a_shop",
                    "goods": goods,
                    "_done": false,
                }
            },
        }),
    );

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_buy_good(Json(payload): JSON) -> JSON {
    let select = payload["select"][0].as_str().unwrap();

    let mut rlv2 = read_json(RLV2_JSON_PATH);
    let item_id = rlv2["player"]["pending"][0]["content"]["shop"]["goods"][select]["itemId"]
        .as_str()
        .unwrap();
    if item_id.contains("_recruit_ticket_") {
        let tkt_id = get_next_tkt(&rlv2);
        add_ticket(&mut rlv2, &tkt_id);
        activate_tkt(&mut rlv2, &tkt_id);
    } else if item_id.contains("_relic_") {
        let relic_id = get_next_relic_id(&rlv2);
        rlv2["inventory"]["relic"][relic_id] = json!({
            "index": relic_id,
            "id": item_id,
            "count": 1,
            "ts": 1695000000,
        });
    } else if item_id.contains("_active_tool_") {
        rlv2["inventory"]["trap"] = json!({
            "index": item_id,
            "id": item_id,
            "count": 1,
            "ts": 1695000000,
        });
    } else if item_id.contains("_explore_tool_") {
        let explore_tool_id = get_next_explore_tool_id(&rlv2);
        rlv2["inventory"]["exploreTool"][&explore_tool_id] = json!({
            "index": explore_tool_id,
            "id": item_id,
            "count": 1,
            "ts": 1695000000,
        });
    }
    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_leave_shop() -> JSON {
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("WAIT_MOVE");
    rlv2["player"]["pending"].as_array_mut().unwrap().clear();

    match rlv2["player"]["cursor"]["position"]["x"].as_i64().unwrap() {
        1 => {
            rlv2["player"]["cursor"]["position"]["x"] = json!(0);
            rlv2["player"]["cursor"]["position"]["y"] = json!(0);
            rlv2["player"]["trace"].as_array_mut().unwrap().pop();
        }
        _ => {
            let zone = rlv2["player"]["cursor"]["zone"].as_i64().unwrap();
            let zone = zone + 1;
            rlv2["player"]["cursor"]["zone"] = json!(zone);
            rlv2["player"]["cursor"]["position"] = Value::Null;
        }
    };

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}

pub async fn rlv2_choose_battle_reward(Json(payload): JSON) -> JSON {
    let index = payload["index"].as_i64().unwrap();
    let mut rlv2 = read_json(RLV2_JSON_PATH);

    if index == 0 {
        let tkt = get_next_tkt(&rlv2);
        add_ticket(&mut rlv2, &tkt);
        activate_tkt(&mut rlv2, &tkt);
    }

    write_json(RLV2_JSON_PATH, &rlv2);

    Json(json!({
        "playerDataDelta": {
            "modified": {
                "rlv2": {
                    "current": rlv2,
                }
            },
            "deleted": {},
        }
    }))
}
