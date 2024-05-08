use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants::{
        config::CONFIG_JSON_PATH,
        url::RL_TABLE_URL,
        user::{RLV2_JSON_PATH, RLV2_USER_SETTINGS_PATH, USER_JSON_PATH},
    },
    utils::{
        enumerate,
        game::update_data,
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

async fn get_map(theme: &str) -> Value {
    const X_MAX: u8 = 9;
    const Y_MAX: u8 = 3;

    let rl_table = update_data(RL_TABLE_URL).await;
    let mut stages = vec![];
    for stage in get_keys(&rl_table["details"][theme]["stages"]) {
        stages.push(stage)
    }
    let shop = match theme {
        "rogue_1" => 8,
        "rogue_2" => 4096,
        "rogue_3" => 4096,
        _ => 0,
    };
    let mut map = json!({});
    let mut zone = 1;
    let mut index = 0;

    while index < stages.len() {
        let mut zone_map = json!({
            "id": format!("zone_{zone}"),
            "index": zone,
            "nodes": {},
            "variation": []
        });
        let mut node_list = vec![
            json!({
                "index": "0",
                "pos": {"x": 0, "y": 0},
                "next": [{"x": 1, "y": 0}],
                "type": shop,
            }),
            json!({
                "index": "100",
                "pos": {"x": 1, "y": 0},
                "next": [],
                "type": shop
            }),
        ];
        let mut x = 1;
        let mut y = 1;
        while index < stages.len() {
            let stage = stages.get(index).unwrap();
            if y > Y_MAX {
                if x + 1 == X_MAX {
                    break;
                }
                x += 1;
                y = 0;
            }
            let mut node_type = 1;
            if rl_table["details"][theme]["stages"][stage]["isElite"].as_bool().unwrap() {
                node_type = 2;
            } else if rl_table["details"][theme]["stages"][stage]["isBoss"].as_bool().unwrap() {
                node_type = 4;
            }
            node_list.push(json!({
                "index": format!("{x}0{y}"),
                "pos": {"x": x, "y": y},
                "next": [],
                "type": node_type,
                "stage": stage,
            }));
            let next_nodes = node_list[0]["next"].as_array_mut().unwrap();
            next_nodes.push(json!({"x": x, "y": y}));
            node_list[0]["next"] = json!(next_nodes);
            y += 1;
            index += 1;
        }
        x += 1;
        let next_nodes = node_list[0]["next"].as_array_mut().unwrap();
        next_nodes.push(json!({"x": x, "y": y}));
        node_list[0]["next"] = json!(next_nodes);
        node_list.push(json!({
            "index": format!("{x}00"),
            "pos": {"x": x, "y": 0},
            "next": [],
            "type": shop,
            "zone_end": true,
        }));
        for node in node_list {
            zone_map["nodes"][node["index"].as_str().unwrap()] = json!(node);
        }
        map[zone.to_string()] = zone_map;
        zone += 1;
    }

    map
}

pub async fn rlv2_finish_event() -> JSON {
    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("WAIT_MOVE");
    rlv2["player"]["cursor"]["zone"] = json!(1);
    rlv2["player"]["pending"] = json!([]);
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

fn get_zone(stage_id: &str) -> i64 {
    let settings = read_json(RLV2_USER_SETTINGS_PATH);
    if settings["stageZone"].get(stage_id).is_some() {
        return settings["stageZone"][stage_id].as_i64().unwrap();
    }
    if stage_id.contains("_n_") && stage_id.contains("_e_") {
        match stage_id.split('_').collect::<Vec<&str>>()[2].parse() {
            Ok(n) => return n,
            Err(_) => return -1,
        };
    }
    -1
}

async fn get_buffs(rlv2: &Value, stage_id: &str) -> Vec<Value> {
    let rl_table = update_data(RL_TABLE_URL).await;
    let theme = rlv2["game"]["theme"].as_str().unwrap();

    let mut buffs = vec![];

    if !rlv2["inventory"]["trap"].is_null() {
        let item_id = rlv2["inventory"]["trap"]["id"].as_str().unwrap();
        if get_keys(&rl_table["details"][theme]["relics"]).contains(&String::from(item_id)) {
            for buff in rl_table["details"][theme]["relics"][item_id]["buffs"].as_array().unwrap() {
                buffs.push(buff.clone());
            }
        }
    }
    for tool in get_keys(&rlv2["inventory"]["exploreTool"]) {
        let item_id = rl_table["inventory"]["exploreTool"][tool]["id"].as_str().unwrap();
        if get_keys(&rl_table["details"][theme]["relics"]).contains(&String::from(item_id)) {
            for buff in rl_table["details"][theme]["relics"][item_id]["buffs"].as_array().unwrap() {
                buffs.push(buff.clone());
            }
        }
    }
    for buff in get_keys(&rlv2["buff"]["squadBuff"]) {
        if get_keys(&rl_table["details"][theme]["squadBuffData"]).contains(&buff) {
            for buff in rl_table["details"][theme]["squadBuffData"][&buff]["buffs"].as_array().unwrap() {
                buffs.push(buff.clone());
            }
        }
    }
    let mode_grade = rlv2["game"]["modeGrade"].as_i64().unwrap();

    let mut theme_buffs = match theme {
        "rogue_1" | "rogue_2" | "rogue_3" => read_json(&format!("./data/rlv2/themeBuffs/buffs_{theme}.json")),
        _ => json!([]),
    };
    for i in 0..theme_buffs.as_array().unwrap().len() {
        if mode_grade < i as i64 {
            break;
        }
        for j in theme_buffs[i][0]
            .clone()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap_or(0) as usize)
        {
            theme_buffs[j] = json!([[], []]);
        }
    }
    for i in 0..theme_buffs.as_array().unwrap().len() {
        if mode_grade < i as i64 {
            break;
        }
        buffs.push(theme_buffs[i][0].clone());
    }
    let mut zone = get_zone(stage_id);
    match theme {
        "rogue_1" => (),
        "rogue_2" => {
            if zone == -1 {
                zone = 6;
            }
            if mode_grade > 0 {
                let val = 1_f64 + 0.01 * mode_grade as f64;
                for _ in 0..zone {
                    buffs.append(&mut vec![
                        json!({
                            "key": "global_buff_normal",
                            "blackboard": [
                                {"key": "key", "valueStr": "enemy_atk_down"},
                                {"key": "atk", "value": val},
                            ],
                        }),
                        json!({
                            "key": "global_buff_normal",
                            "blackboard": [
                                {"key": "key", "valueStr": "enemy_max_hp_down"},
                                {"key": "max_hp", "value": val},
                            ],
                        }),
                    ])
                }
            }
        }
        "rogue_3" => {
            if zone == -1 {
                zone = 7;
            }
            if mode_grade > 4 {
                let val = 1_f64 + 0.16 * (mode_grade - 4) as f64 / 11_f64;
                for _ in 0..zone {
                    buffs.append(&mut vec![
                        json!({
                            "key": "global_buff_normal",
                            "blackboard": [
                                {"key": "key", "valueStr": "enemy_atk_down"},
                                {"key": "atk", "value": val},
                            ],
                        }),
                        json!({
                            "key": "global_buff_normal",
                            "blackboard": [
                                {"key": "key", "valueStr": "enemy_max_hp_down"},
                                {"key": "max_hp", "value": val},
                            ],
                        }),
                    ])
                }
            }
        }
        _ => (),
    }
    buffs
}

pub async fn rlve2_mv_n_battle_start(Json(payload): JSON) -> JSON {
    let stage_id = payload["stageId"].as_str().unwrap();
    let x = payload["to"]["x"].as_u64().unwrap();
    let y = payload["to"]["y"].as_u64().unwrap();

    let mut rlv2 = read_json(RLV2_JSON_PATH);
    rlv2["player"]["state"] = json!("PENDING");
    rlv2["player"]["cursor"]["position"] = json!({"x": x, "y": y});

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
