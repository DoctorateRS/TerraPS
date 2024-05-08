use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants::{
        config::CONFIG_JSON_PATH,
        user::{RLV2_JSON_PATH, RLV2_USER_SETTINGS_PATH, USER_JSON_PATH},
    },
    utils::{
        enumerate,
        json::{get_keys, read_json, write_json, JSON},
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

fn get_chars(default: bool) -> Vec<Value> {
    let user_data = read_json(USER_JSON_PATH);
    let mut chars = vec![];
    for char in get_keys(&user_data["user"]["troop"]["chars"]) {
        chars.push(user_data["user"]["troop"]["chars"][&char].clone())
    }
    if default {
        let rlv2_user_settings = read_json(RLV2_USER_SETTINGS_PATH);
        let init_chars = rlv2_user_settings["initialChars"].as_array().unwrap();
        let _ = chars
            .iter()
            .filter(|char| (init_chars.contains(char)))
            .cloned()
            .collect::<Vec<Value>>();
    }

    let mut res = vec![];

    for (id, mut char) in enumerate(chars) {
        let ep = char["evolvePhase"].as_i64().unwrap();
        char["instId"] = json!(id.to_string());
        char["type"] = json!("NORMAL");
        char["upgradeLimited"] = if ep < 2 { json!(true) } else { json!(false) };
        char["upgradePhase"] = if ep < 2 { json!(0) } else { json!(1) };
        char["isUpgrade"] = json!(false);
        char["isCure"] = json!(false);
        char["population"] = json!(0);
        char["charBuff"] = json!([]);
        char["troopInstId"] = json!("0");
        res.push(char);
    }

    res
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
                "gold": 450,
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
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    let mut rlv2_pend_vec = rlv2["player"]["pending"].clone();
    let rlv2_pend_vec = rlv2_pend_vec.as_array_mut().unwrap();

    let band = rlv2["player"]["pending"][0]["content"]["initRelic"]["items"][select]["id"]
        .as_str()
        .unwrap();
    rlv2_pend_vec.remove(0);

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

fn add_ticket(rlv2: &mut Value, ticket_id: &str) {
    let theme = rlv2["game"]["theme"].as_str().unwrap();
    let ticket = match theme {
        "rogue_1" => "rogue_1_recruit_ticket_all",
        "rogue_2" => "rogue_2_recruit_ticket_all",
        "rogue_3" => "rogue_3_recruit_ticket_all",
        _ => "",
    };
    rlv2["inventory"]["recruit"][ticket_id] = json!({
        "index": ticket_id,
        "id": ticket,
        "state": 0,
        "list": [],
        "result": Value::Null,
        "ts": 1695000000,
        "from": "initial",
        "mustExtra": 0,
        "needAssist": true,
    });
}

fn get_next_tkt(rlv2: &Value) -> String {
    let mut v = vec![];
    for mut tkt in get_keys(&rlv2["inventory"]["recruit"]) {
        v.push(tkt.split_off(1).parse::<usize>().unwrap_or(0));
    }
    let config = read_json(CONFIG_JSON_PATH);
    let mut index = if !config["rlv2Config"]["allChars"].as_bool().unwrap() {
        0
    } else {
        10000 - 1
    };
    while v.contains(&index) {
        index += 1;
    }
    format!("t_{}", index)
}

pub async fn rlv2_choose_init_recruit_set() -> JSON {
    let mut rlv2 = read_json(RLV2_JSON_PATH);

    let pending = rlv2["player"]["pending"].as_array_mut().unwrap();
    pending.remove(0);

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

fn get_next_pending(rlv2: &Value) -> String {
    let mut v = vec![];
    for p in rlv2["player"]["pending"].as_array().unwrap() {
        v.push(p["index"].as_str().unwrap().split_at(1).1.parse::<usize>().unwrap_or(0));
    }
    let mut i = 0;
    while v.contains(&i) {
        i += 1;
    }
    format!("e_{}", i)
}

fn activate_tkt(rlv2: &mut Value, tkt_id: &str) {
    let pending_index = get_next_pending(rlv2);
    let pending_arr = rlv2["player"]["pending"].as_array_mut().unwrap();
    pending_arr.insert(
        0,
        json!({
            "index": pending_index,
            "type": "RECRUIT",
            "content": {"recruit": {"ticket": tkt_id}},
        }),
    );
    let chars = get_chars(false);
    rlv2["inventory"]["recruit"][tkt_id]["state"] = json!(1);
    rlv2["inventory"]["recruit"][tkt_id]["list"] = json!(chars);
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

fn get_next_char_id(rlv2: &Value) -> String {
    let config = read_json(CONFIG_JSON_PATH);
    let mut i = if !config["rlv2Config"]["allChars"].as_bool().unwrap() {
        1
    } else {
        10000
    };
    while get_keys(&rlv2["troop"]["chars"]).contains(&i.to_string()) {
        i += 1;
    }
    i.to_string()
}

pub async fn rlv2_recruit_char(Json(payload): JSON) -> JSON {
    let tkt_id = payload["ticketIndex"].as_str().unwrap();
    let opt_id = payload["optionId"].as_str().unwrap().parse::<usize>().unwrap();
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    let rlv2_pending = rlv2["player"]["pending"].as_array_mut().unwrap();
    rlv2_pending.remove(0);
    rlv2["player"]["pending"] = json!(rlv2_pending);

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
    let rlv2_pending = rlv2["player"]["pending"].as_array_mut().unwrap();
    rlv2_pending.remove(0);
    rlv2["player"]["pending"] = json!(rlv2_pending);
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
