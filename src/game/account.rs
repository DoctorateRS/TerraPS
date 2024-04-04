use crate::{
    constants::{
        config::{CONFIG_JSON_PATH, MAILLIST_PATH, SYNC_DATA_TEMPLATE_PATH},
        url::{
            ACTIVITY_TABLE_URL, BATTLEEQUIP_TABLE_URL, CHARACTER_TABLE_URL, CHARM_TABLE_URL, CHARWORD_TABLE_URL, DM_TABLE_URL, EQUIP_TABLE_URL,
            HANDBOOK_INFO_TABLE_URL, RETRO_TABLE_URL, SKIN_TABLE_URL, STORY_TABLE_URL,
        },
        user::USER_JSON_PATH,
    },
    core::{time, JSON},
    utils::{contains, get_keys, get_values, max, read_json, update_data, zipper},
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

    let mut count = 0;
    let mut count_inst_id;
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
            "defaultSkillIndex": char_table[&operator]["skills"].as_array().unwrap().len() as i64 - 1,
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
            for equip in get_keys(&equip_table["charEquip"][temp_char_list[count_inst_id.to_string()]["charId"].to_string()]) {
                let mut lvl = 1;
                if contains(&equip, get_keys(&battleequip_table)) {
                    lvl = battleequip_table[&equip]["phases"].as_array().unwrap().len();
                }
                temp_char_list[count_inst_id.to_string()]["equip"] = json!({
                    equip: {
                        "hide": 0,
                        "locked": 0,
                        "level": lvl
                    }
                });
            }
            temp_char_list[count_inst_id.to_string()]["currentEquip"] = equip_table["charEquip"]
                [temp_char_list[count_inst_id.to_string()]["charId"].to_string()]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .clone();
        }
        player_data["user"]["dexNav"]["character"][&operator_keys[count]] = json!({
            "charInstId": count_inst_id,
            "count": 6
        });

        if operator_keys[count] == "char_002_amiya" {
            temp_char_list[count_inst_id.to_string()] = json!({
                "defaultSkillIndex": -1,
                "skills": [],
                "currentTmpl": "char_002_amiya",
                "tmpl": {
                    "char_002_amiya": {
                        "skinId": "char_002_amiya@test#1",
                        "defaultSkillIndex": 2,
                        "skills": [
                            {
                                "skillId": "skchr_amiya_3",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": operator_template["skillsSpecializeLevel"],
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya_2",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": operator_template["skillsSpecializeLevel"],
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skcom_magic_rage[3]",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": operator_template["skillsSpecializeLevel"],
                                "completeUpgradeTime": -1
                            }
                        ],
                        "currentEquip": Value::Null,
                        "equip": {},
                    },
                    "char_1001_amiya2": {
                        "skinId": "char_1001_amiya2@casc#1",
                        "defaultSkillIndex": 1,
                        "skills": [
                            {
                                "skillId": "skchr_amiya2_1",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": operator_template["skillsSpecializeLevel"],
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya2_2",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": operator_template["skillsSpecializeLevel"],
                                "completeUpgradeTime": -1
                            }
                        ],
                        "currentEquip": Value::Null,
                        "equip": {},
                    }
                }
            });
            for equip in get_keys(&equip_table["charEquip"]["char_002_amiya"]) {
                let mut lvl = 1;
                if contains(&equip, get_keys(&battleequip_table)) {
                    lvl = battleequip_table[&equip]["phases"].as_array().unwrap().len();
                }
                temp_char_list[count_inst_id.to_string()]["tmpl"]["char_002_amiya"]["equip"] = json!({
                    equip: {
                        "hide": 0,
                        "locked": 0,
                        "level": lvl
                    }
                });
            }
            temp_char_list[count_inst_id.to_string()]["tmpl"]["char_002_amiya"]["currentEquip"] = equip_table["charEquip"]["char_002_amiya"]
                .as_array()
                .unwrap()
                .last()
                .unwrap()
                .clone();
        } else if operator_keys[count] == "char_512_aprot" {
            temp_char_list[count_inst_id.to_string()]["skin"] = json!("char_512_aprot#1");
        }

        building_chars[count_inst_id.to_string()] = json!({
            "charId": operator_keys[count],
            "lastApAddTime": time() - 3600,
            "ap": 8640000,
            "roomSlotId": "",
            "index": -1,
            "changeScale": 0,
            "bubble": {
                "normal": {
                    "add": -1,
                    "ts": 0
                },
                "assist": {
                    "add": -1,
                    "ts": 0
                }
            },
            "workTime": 0
        });

        count += 1;
    }

    let count_inst_id = 10000;

    player_data["user"]["troop"]["chars"] = temp_char_list;
    player_data["user"]["troop"]["charGroup"] = char_group;
    player_data["user"]["troop"]["curCharInstId"] = json!(count_inst_id);

    // Story
    let mut story_list = json!({"init": 1});
    let story_table = update_data(STORY_TABLE_URL).await;
    for story in get_keys(&story_table) {
        story_list[story] = json!(1);
    }

    player_data["user"]["status"]["flags"] = story_list;

    // Stages
    let mut stage_list = json!({});
    let stage_table = update_data(STORY_TABLE_URL).await;
    for stage in get_keys(&stage_table) {
        stage_list[&stage] = json!({
            "completeTimes": 1,
            "hasBattleReplay": 0,
            "noCostCnt": 0,
            "practiceTimes": 0,
            "stageId": stage_table["stages"][&stage]["stageId"],
            "startTimes": 1,
            "state": 3
        });
    }

    player_data["user"]["status"]["stages"] = stage_list;

    // Addons
    let mut addon_list = json!({});
    let addon_table = update_data(HANDBOOK_INFO_TABLE_URL).await;
    for char_id in get_keys(&addon_table["handbookDict"]) {
        addon_list[&char_id] = json!({"story":{}});
        let story = addon_table["handbookDict"][&char_id]["handbookAvgList"].clone();
        for (story_keys, id) in zipper(get_keys(&story), 0..story.as_array().unwrap().len()) {
            if story_keys.contains("storySetId") {
                let story_set_id = addon_table["handbookDict"][&char_id]["handbookAvgList"].as_array().unwrap()[id]["storySetId"]
                    .as_str()
                    .unwrap();
                addon_list[&char_id]["story"] = json!({
                    story_set_id: {
                        "fts": 1649232340,
                        "rts": 1649232340
                    }
                });
            }
        }
    }
    for stage in get_keys(&addon_table["handbookStageData"]) {
        let stage_id = addon_table["handbookStageData"][&stage]["stageId"].as_str().unwrap();
        addon_list[&stage] = json!({
            "stage": {
                stage_id: {
                    "startTimes": 0,
                    "completeTimes": 1,
                    "state": 3,
                    "fts": 1624284657,
                    "rts": 1624284657,
                    "startTime": 2
                }
            }
        });
    }

    player_data["user"]["troop"]["addon"] = addon_list;

    // Retrospective
    let mut blocks = json!({});
    for retro in get_keys(&retro_table["retroActList"]) {
        blocks[retro] = json!({
            "locked": 0,
            "open": 1
        });
    }
    player_data["user"]["retro"]["block"] = blocks;

    let mut trails = json!({});
    for retro in get_keys(&retro_table["retroTrailList"]) {
        trails[&retro] = json!({});
        for trail_reward in retro_table["retroTrailList"][&retro]["trailRewardList"].as_array().unwrap() {
            trails[&retro][trail_reward["trailRewardId"].as_str().unwrap()] = json!(1);
        }
    }
    player_data["user"]["retro"]["trail"] = trails;

    for stage in get_keys(&stage_table["stages"]) {
        if stage.starts_with("camp") {
            player_data["user"]["campaignsV2"]["instances"][&stage] = json!({
                            "maxKills": 400,
                            "rewardStatus": [1, 1, 1, 1, 1, 1, 1, 1]
                        }
            );

            let new_perma_vec = player_data["user"]["campaignsV2"]["open"]["permanent"].clone();
            let mut new_perma_vec = new_perma_vec.as_array().unwrap().clone();
            let new_training_vec = player_data["user"]["campaignsV2"]["open"]["training"].clone();
            let mut new_training_vec = new_training_vec.as_array().unwrap().clone();
            new_perma_vec.push(json!(stage));
            new_training_vec.push(json!(stage));

            player_data["user"]["campaignsV2"]["sweepMaxKills"][&stage] = json!(400);
            player_data["user"]["campaignsV2"]["open"]["permanent"] = json!(new_perma_vec);
            player_data["user"]["campaignsV2"]["open"]["training"] = json!(new_training_vec);
        }
    }

    let mut avatar_list = json!({});

    for avatar in display_meta_table["playerAvatarData"]["avatarList"].as_array().unwrap() {
        let id = avatar["avatarId"].as_str().unwrap();
        let src = if id.starts_with("avatar_def") { "initial" } else { "other" };
        avatar_list[&id] = json!({
            "ts": time(),
            "src": src,
        });
    }

    player_data["user"]["avatar"]["avatar_icon"] = avatar_list;

    let mut bg_list = json!({});

    for bg in display_meta_table["homeBackgroundData"]["homeBgDataList"].as_array().unwrap() {
        let id = bg["bgId"].as_str().unwrap();
        bg_list[&id] = json!({
            "unlock": time(),
        });
    }
    player_data["user"]["background"]["bgs"] = bg_list;

    let mut theme_list = json!({});
    for theme in display_meta_table["homeBackgroundData"]["themeList"].as_array().unwrap() {
        let id = theme["id"].as_str().unwrap();
        theme_list[&id] = json!({
            "unlock": time(),
        });
    }
    player_data["user"]["homeTheme"]["themes"] = theme_list;

    for charm in charm_table["charmList"].as_array().unwrap() {
        let id = charm["id"].as_str().unwrap();
        player_data["user"]["charm"]["charms"][id] = json!(1);
    }

    // SN & IC code here

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
