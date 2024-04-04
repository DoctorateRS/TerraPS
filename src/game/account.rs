use crate::{
    constants::{
        config::{CONFIG_JSON_PATH, MAILLIST_PATH, SYNC_DATA_TEMPLATE_PATH},
        url::{
            ACTIVITY_TABLE_URL, BATTLEEQUIP_TABLE_URL, CHARACTER_TABLE_URL, CHARM_TABLE_URL, CHARWORD_TABLE_URL, DM_TABLE_URL, EQUIP_TABLE_URL,
            RETRO_TABLE_URL, SKIN_TABLE_URL,
        },
        user::USER_JSON_PATH,
    },
    core::{time, JSON},
    utils::{contains, get_keys, get_values, max, read_json, update_data},
};
use axum::{http::HeaderMap, Json};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn account_login(header: HeaderMap) -> JSON {
    let fallback_uid = Uuid::new_v4().to_string();
    let uid = match header.get("Uid") {
        Some(uid) => uid.to_str().unwrap_or(fallback_uid.as_str()),
        None => fallback_uid.as_str(),
    };
    Json(json!({
        "result": 0,
        "uid": uid,
        "secret": "yostar",
        "serviceLicenseVersion": 0
    }))
}

pub async fn account_sync_data() -> JSON {
    let user_data = read_json(USER_JSON_PATH);
    let mail_data = read_json(MAILLIST_PATH);
    let mut player_data = read_json(SYNC_DATA_TEMPLATE_PATH);
    let config = read_json(CONFIG_JSON_PATH);

    // Loading latest data

    let skin_table = update_data(SKIN_TABLE_URL).await;
    let char_table = update_data(CHARACTER_TABLE_URL).await;
    let equip_table = update_data(EQUIP_TABLE_URL).await;
    let battleequip_table = update_data(BATTLEEQUIP_TABLE_URL).await;
    let display_meta_table = update_data(DM_TABLE_URL).await;
    let retro_table = update_data(RETRO_TABLE_URL).await;
    let charm_table = update_data(CHARM_TABLE_URL).await;
    let acitivity_table = update_data(ACTIVITY_TABLE_URL).await;
    let charword_table = update_data(CHARWORD_TABLE_URL).await;

    let ts = time();
    let mut count = 0;
    let mut count_inst_id = 1;
    let mut max_inst_id = 1;

    // Generics

    let mut temp_skin_table = json!({});
    let mut temp_char_list = json!({});
    let mut char_group = json!({});
    let mut building_chars = json!({});

    // Skin
    let skin_keys = get_keys(&skin_table);
    player_data["user"]["skin"]["characterSkins"] = json!({});

    for skin_data in get_values(&skin_table) {
        if !&skin_keys[count].contains('@') {
            count += 1;
            continue;
        }
        player_data["user"]["skin"]["characterSkins"][&skin_keys[count]] = json!(1);
        if !contains(&skin_data["charId"].as_str().unwrap().to_string(), get_keys(&temp_skin_table))
            || skin_data["displaySkin"]["onYear"].as_i64().unwrap()
                > skin_table["charSkins"][&temp_skin_table[&skin_data["charId"].as_str().unwrap()].as_str().unwrap()]["displaySkin"]["onYear"]
                    .as_i64()
                    .unwrap()
        {
            temp_skin_table[&skin_data["charId"].as_str().unwrap()] = json!(skin_keys[count].clone());
        }
        count += 1;
    }

    // Operators
    let operator_template = config["charConfig"].clone();

    let mut count = 0;
    let operator_keys = get_keys(&char_table);
    let equip_keys = get_keys(&equip_table["charEquip"]);

    for operator in &operator_keys {
        if !operator.contains("char") {
            continue;
        }
        char_group[operator] = json!({"favorPoint": 25570})
    }

    for operator in get_keys(&char_table) {
        if !operator_keys[count].contains("char") {
            count += 1;
            continue;
        }

        // Add all operators to the player data

        let evolve_phase = match operator_template["evolvePhase"].as_i64() {
            Some(evolve_phase) => match evolve_phase {
                ..=0 => char_table[&operator]["phases"].as_array().unwrap().len() as i64 - 1,
                _ => evolve_phase,
            },
            None => panic!("Invalid elite status."),
        };

        let level = match operator_template["level"].as_i64() {
            Some(level) => match level {
                ..=0 => char_table[&operator]["phases"][evolve_phase as usize]["maxLevel"].as_u64().unwrap(),
                _ => level as u64,
            },
            None => panic!("Invalid level."),
        };

        count_inst_id = operator_keys[count].split('_').collect::<Vec<&str>>()[1].parse().unwrap();
        max_inst_id = max(max_inst_id, count_inst_id);
        let mut voice_lan = "JP";
        if contains(&operator_keys[count], get_keys(&charword_table)) {
            voice_lan = charword_table["charDefaultTypeDict"][&operator_keys[count]].as_str().unwrap();
        }

        temp_char_list[count_inst_id.to_string()] = json!({
            "instId": count_inst_id,
            "charId": operator_keys[count],
            "favorPoint": operator_template["favorPoint"],
            "potentialRank": operator_template["potentialRank"],
            "mainSkillLvl": operator_template["mainSkillLvl"],
            "skin": operator_keys[count].clone() + "#1",
            "level": level,
            "exp": 0,
            "evolvePhase": evolve_phase,
            "defaultSkillIndex": char_table[&operator]["skills"].as_array().unwrap().len() as u64 - 1,
            "gainTime": time(),
            "skills": [],
            "voiceLan": voice_lan,
            "currentEquip": Value::Null,
            "equip": {},
            "starMark": 0
        });

        // Set E2 skin

        if contains(
            &operator_keys[count],
            vec![
                "char_508_aguard".to_string(),
                "char_509_acast".to_string(),
                "char_510_amedic".to_string(),
                "char_511_asnipe".to_string(),
            ],
        ) && temp_char_list[count_inst_id.to_string()]["evolvePhase"].as_i64().unwrap() == 2
        {
            temp_char_list[count_inst_id.to_string()]["skin"] = json!(operator_keys[count].clone() + "#2");
        }

        // Set skins
        if contains(&operator_keys[count], get_keys(&temp_skin_table)) {
            temp_char_list[count_inst_id.to_string()]["skin"] = json!(temp_skin_table[&operator_keys[count]]);
        }

        let mut skill_vec = Vec::new();
        // Set skills
        for skill in char_table[&operator]["skills"].as_array().unwrap() {
            let specialization_level = if !skill["levelUpCostCond"].as_array().unwrap().is_empty() {
                operator_template["skillsSpecializeLevel"].as_u64().unwrap()
            } else {
                0
            };
            skill_vec.push(json!({
                "skillId": skill["skillId"],
                "unlock": 1,
                "state": 0,
                "specializeLevel": specialization_level,
                "completeUpgradeTime": -1
            }));
        }
        temp_char_list[count_inst_id.to_string()]["skills"] = json!(skill_vec);

        // Set modules
        if contains(&temp_char_list[count_inst_id.to_string()]["charId"].to_string(), equip_keys.clone()) {
            for equip in equip_table["charEquip"][temp_char_list[count_inst_id.to_string()]] {}
        }
    }

    Json(player_data)
}

pub async fn account_sync_status() -> JSON {
    Json(json!({
        "ts": time(),
        "result": {},
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn account_yostar_auth_req() -> JSON {
    Json(json!({}))
}

pub async fn account_yostar_auth_submit() -> JSON {
    Json(json!({
        "result": 0,
        "yostar_account": "Doctorate@doctorate.com",
        "yostar_token": "a",
        "yostar_uid": "1"
    }))
}
