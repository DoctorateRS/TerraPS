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
    utils::{contains, get_keys, get_values, read_json, update_data},
};
use axum::{http::HeaderMap, Json};
use serde_json::json;
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
        if !contains(skin_data["charId"].as_str().unwrap().to_string(), get_keys(&temp_skin_table))
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
