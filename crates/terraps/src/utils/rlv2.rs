use serde_json::{json, Value};

use crate::{
    constants::{
        config::CONFIG_JSON_PATH,
        url::RL_TABLE_URL,
        user::{RLV2_USER_SETTINGS_PATH, USER_JSON_PATH},
    },
    utils::game::update_data,
};

use super::{enumerate, json::get_keys};

use common_utils::read_json;

pub fn get_chars(default: bool) -> Vec<Value> {
    let user_data = read_json(USER_JSON_PATH);
    let mut chars = vec![];
    for char in get_keys(&user_data["user"]["troop"]["chars"]) {
        chars.push(user_data["user"]["troop"]["chars"][&char].clone())
    }
    if default {
        let rlv2_user_settings = read_json(RLV2_USER_SETTINGS_PATH);
        let init_chars = rlv2_user_settings["initialChars"].as_array().unwrap();
        let _ = chars.iter().filter(|char| (init_chars.contains(char))).cloned().collect::<Vec<Value>>();
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

pub fn add_ticket(rlv2: &mut Value, ticket_id: &str) {
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
        "result": null,
        "ts": 1695000000,
        "from": "initial",
        "mustExtra": 0,
        "needAssist": true,
    });
}

pub fn get_next_tkt(rlv2: &Value) -> String {
    let mut v = vec![];
    for mut tkt in get_keys(&rlv2["inventory"]["recruit"]) {
        v.push(tkt.split_off(1).parse::<usize>().unwrap_or(0));
    }
    let config = read_json(CONFIG_JSON_PATH);
    let mut index = if !config["rlv2Config"]["allChars"].as_bool().unwrap() { 0 } else { 10000 - 1 };
    while v.contains(&index) {
        index += 1;
    }
    format!("t_{}", index)
}

pub fn get_next_pending(rlv2: &Value) -> String {
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

pub fn activate_tkt(rlv2: &mut Value, tkt_id: &str) {
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

pub fn get_next_char_id(rlv2: &Value) -> String {
    let config = read_json(CONFIG_JSON_PATH);
    let mut i = if !config["rlv2Config"]["allChars"].as_bool().unwrap() { 1 } else { 10000 };
    while get_keys(&rlv2["troop"]["chars"]).contains(&i.to_string()) {
        i += 1;
    }
    i.to_string()
}

pub async fn get_map(theme: &str) -> Value {
    const X_MAX: u8 = 9;
    const Y_MAX: u8 = 3;

    let rl_table = update_data(RL_TABLE_URL).await;
    let mut stages = vec![];
    for stage in get_keys(&rl_table["details"][theme]["stages"]) {
        stages.push(stage)
    }
    let shop = match theme {
        "rogue_1" => 8,
        "rogue_2" | "rogue_3" => 4096,
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
            if rl_table["details"][theme]["stages"][stage]["isElite"].as_i64().unwrap() != 0 {
                node_type = 2;
            } else if rl_table["details"][theme]["stages"][stage]["isBoss"].as_i64().unwrap() != 0 {
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
pub fn get_zone(stage_id: &str) -> i64 {
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

pub async fn get_buffs(rlv2: &Value, stage_id: &str) -> Vec<Value> {
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
        "rogue_1" | "rogue_2" | "rogue_3" => read_json(format!("./data/rlv2/themeBuffs/buffs_{theme}.json")),
        _ => json!([]),
    };
    for i in 0..theme_buffs.as_array().unwrap().len() {
        if mode_grade < i as i64 {
            break;
        }
        for j in theme_buffs[i][0].clone().as_array().unwrap().iter().map(|x| x.as_u64().unwrap_or(0) as usize) {
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

pub async fn get_goods(theme: &str) -> Vec<Value> {
    let (ticket, price_id) = match theme {
        "rogue_1" => ("rogue_1_recruit_ticket_all", "rogue_1_gold"),
        "rogue_2" => ("rogue_2_recruit_ticket_all", "rogue_2_gold"),
        "rogue_3" => ("rogue_3_recruit_ticket_all", "rogue_3_gold"),
        _ => ("", ""),
    };
    let mut goods = vec![json!({
        "index": "0",
        "itemId": ticket,
        "count": 1,
        "priceId": price_id,
        "priceCount": 0,
        "origCost": 0,
        "displayPriceChg": false,
        "_retainDiscount": 1,
    })];
    let mut id = 1;
    let rl_table = update_data(RL_TABLE_URL).await;
    for item in get_keys(&rl_table["details"][theme]["archiveComp"]["relic"]["relic"]) {
        goods.push(json!({
            "index": id.to_string(),
            "itemId": item,
            "count": 1,
            "priceId": price_id,
            "priceCount": 0,
            "origCost": 0,
            "displayPriceChg": false,
            "_retainDiscount": 1,
        }));
        id += 1;
    }
    for item in get_keys(&rl_table["details"][theme]["difficultyUpgradeRelicGroups"]) {
        for relic in get_keys(&rl_table["details"][theme]["difficultyUpgradeRelicGroups"][&item]["relicData"]) {
            let relic = rl_table["details"][theme]["difficultyUpgradeRelicGroups"][&item]["relicData"][relic].clone();
            goods.push(json!({
                "index": id.to_string(),
                "itemId": relic["relicId"],
                "count": 1,
                "priceId": price_id,
                "priceCount": 0,
                "origCost": 0,
                "displayPriceChg": false,
                "_retainDiscount": 1,
            }));
            id += 1;
        }
    }
    for trap in get_keys(&rl_table["details"][theme]["archiveComp"]["trap"]["trap"]) {
        goods.push(json!({
            "index": id.to_string(),
            "itemId": trap,
            "count": 1,
            "priceId": price_id,
            "priceCount": 0,
            "origCost": 0,
            "displayPriceChg": false,
            "_retainDiscount": 1,
        }));
        id += 1;
    }
    goods
}

pub fn get_next_relic_id(rlv2: &Value) -> String {
    let mut v = vec![];
    for p in rlv2["inventory"]["relic"].as_array().unwrap() {
        v.push(p["index"].as_str().unwrap().split_at(1).1.parse::<usize>().unwrap_or(0));
    }
    let mut i = 0;
    while v.contains(&i) {
        i += 1;
    }
    format!("r_{}", i)
}

pub fn get_next_explore_tool_id(rlv2: &Value) -> String {
    let mut v = vec![];
    for p in rlv2["inventory"]["exploreTool"].as_array().unwrap() {
        v.push(p["index"].as_str().unwrap().split_at(1).1.parse::<usize>().unwrap_or(0));
    }
    let mut i = 0;
    while v.contains(&i) {
        i += 1;
    }
    format!("e_{}", i)
}
