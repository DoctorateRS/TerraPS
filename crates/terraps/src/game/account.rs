use crate::{
    constants::{
        config::{CONFIG_JSON_PATH, MAILLIST_PATH, SQUADS_PATH, SYNC_DATA_TEMPLATE_PATH},
        url::*,
        user::{BATTLE_REPLAY_JSON_PATH, CRISIS_V2_JSON_BASE_PATH, USER_JSON_PATH},
    },
    core::time,
    utils::{game::*, get_nickname_config, json::*},
};
use axum::{http::HeaderMap, Json};
use serde_json::json;
use uuid::Uuid;

use common_utils::{read_json, write_json};

use super::building::building_sync;

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
    // let user_data = read_json(USER_JSON_PATH);
    let mail_data = read_json(MAILLIST_PATH);
    let mut player_data = read_json(SYNC_DATA_TEMPLATE_PATH);
    let config = read_json(CONFIG_JSON_PATH);

    let (nick_name, nick_id) = get_nickname_config();

    // Loading latest data

    let skin_table = update_data(SKIN_TABLE_URL).await;
    let char_table = update_data(CHARACTER_TABLE_URL).await;
    let equip_table = update_data(EQUIP_TABLE_URL).await;
    let battleequip_table = update_data(BATTLEEQUIP_TABLE_URL).await;
    let display_meta_table = update_data(DM_TABLE_URL).await;
    let retro_table = update_data(RETRO_TABLE_URL).await;
    let charm_table = update_data(CHARM_TABLE_URL).await;
    let activity_table = update_data(ACTIVITY_TABLE_URL).await;
    let charword_table = update_data(CHARWORD_TABLE_URL).await;
    let story_review_table = update_data(STORY_REVIEW_TABLE_URL).await;
    let story_review_meta_table = update_data(STORY_REVIEW_META_TABLE_URL).await;
    let enemy_handbook_table = update_data(ENEMY_HANDBOOK_TABLE_URL).await;
    let medal_table = update_data(MEDAL_TABLE_URL).await;
    let rlv2_table = update_data(RL_TABLE_URL).await;
    let stage_table = update_data(STAGE_TABLE_URL).await;
    let story_table = update_data(STORY_TABLE_URL).await;

    let mut count = 0;
    let mut count_inst_id: usize;
    let mut max_inst_id = 1;

    // Generics

    let mut temp_skin_table = json!({});
    let mut temp_char_list = json!({});
    let mut char_group = json!({});
    let mut building_chars = json!({});

    // Skin
    let skin_keys = get_keys(&skin_table["charSkins"]);
    player_data["user"]["skin"]["characterSkins"] = json!({});

    for skin_data in get_values(&skin_table["charSkins"]) {
        if !&skin_keys[count].contains('@') {
            count += 1;
            continue;
        }
        player_data["user"]["skin"]["characterSkins"][&skin_keys[count]] = json!(1);
        if { !get_keys(&temp_skin_table).iter().map(|x| x.as_str()).collect::<Vec<&str>>().contains(&skin_data["charId"].as_str().unwrap()) } || {
            skin_data["displaySkin"]["onYear"].as_u64().unwrap() > skin_table["charSkins"][&temp_skin_table[&skin_data["charId"].as_str().unwrap()].as_str().unwrap()]["displaySkin"]["onYear"].as_u64().unwrap()
        } {
            temp_skin_table[skin_data["charId"].as_str().unwrap()] = skin_data["skinId"].clone();
        }
        count += 1;
    }

    // Operators
    let operator_template = &config["charConfig"];
    let operator_keys = get_keys(&char_table);
    let equip_keys = get_keys(&equip_table["charEquip"]);

    for operator in &operator_keys {
        if !operator.contains("char") {
            continue;
        }
        char_group[operator] = json!({"favorPoint": 25570})
    }

    let mut count = 0;

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
        max_inst_id = max_inst_id.max(count_inst_id);
        let mut voice_lan = "JP";
        if get_keys(&charword_table).contains(&operator_keys[count]) {
            voice_lan = charword_table["charDefaultTypeDict"][&operator_keys[count]].as_str().unwrap();
        }

        let cid = count_inst_id.to_string();

        temp_char_list[&cid] = json!({
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
            "currentEquip": null,
            "equip": {},
            "starMark": 0
        });

        // Set E2 skin
        if { !["char_508_aguard", "char_509_acast", "char_510_amedic", "char_511_asnipe"].contains(&operator_keys[count].as_str()) } && { temp_char_list[&cid]["evolvePhase"].as_u64().unwrap() == 2 } {
            temp_char_list[&cid]["skin"] = json!(operator_keys[count].clone() + "#2");
        }

        // Set skins
        if get_keys(&temp_skin_table).contains(&operator_keys[count]) {
            temp_char_list[&cid]["skin"] = json!(temp_skin_table[&operator_keys[count]]);
        }

        // Set skills
        let mut skill_vec = Vec::new();
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
        temp_char_list[&cid]["skills"] = json!(skill_vec);

        // Set modules
        if equip_keys.contains(&temp_char_list[&cid]["charId"].as_str().unwrap().to_string()) {
            let equip_vec = equip_table["charEquip"][temp_char_list[&cid]["charId"].as_str().unwrap()]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>();
            for equip in equip_vec {
                let mut lvl = 1;
                if get_keys(&battleequip_table).contains(&equip) {
                    lvl = battleequip_table[&equip]["phases"].as_array().unwrap().len();
                }
                temp_char_list[&cid]["equip"][&equip] = json!({
                    "hide": 0,
                    "locked": 0,
                    "level": lvl
                });
            }
            temp_char_list[&cid]["currentEquip"] = json!(&equip_table["charEquip"][temp_char_list[&cid]["charId"].as_str().unwrap()].as_array().unwrap().last().unwrap());
        }

        player_data["user"]["dexNav"]["character"][&operator_keys[count]] = json!({
            "charInstId": count_inst_id,
            "count": 6
        });

        if operator_keys[count] == "char_002_amiya" {
            temp_char_list[&cid] = json!({
                "instId": 2,
                "charId": "char_002_amiya",
                "favorPoint": 25570,
                "potentialRank": 5,
                "mainSkillLvl": 7,
                "skin": "char_1001_amiya2@casc#1",
                "level": 80,
                "exp": 0,
                "evolvePhase": 2,
                "defaultSkillIndex": -1,
                "gainTime": 1712627499,
                "skills": [],
                "voiceLan": "JP",
                "currentEquip": "uniequip_002_amiya",
                "equip": {
                    "uniequip_001_amiya": {
                        "hide": 0,
                        "locked": 0,
                        "level": 1
                    },
                    "uniequip_002_amiya": {
                        "hide": 0,
                        "locked": 0,
                        "level": 3
                    }
                },
                "starMark": 0,
                "currentTmpl": "char_002_amiya",
                "tmpl": {
                    "char_002_amiya": {
                        "skinId": "char_002_amiya@test#1",
                        "defaultSkillIndex": 2,
                        "skills": [
                            {
                                "skillId": "skcom_magic_rage[3]",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya_2",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya_3",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            }
                        ],
                        "currentEquip": "uniequip_002_amiya",
                        "equip": {
                            "uniequip_001_amiya": {
                                "hide": 0,
                                "locked": 0,
                                "level": 1
                            },
                            "uniequip_002_amiya": {
                                "hide": 0,
                                "locked": 0,
                                "level": 3
                            }
                        }
                    },
                    "char_1001_amiya2": {
                        "skinId": "char_1001_amiya2@casc#1",
                        "defaultSkillIndex": 1,
                        "skills": [
                            {
                                "skillId": "skchr_amiya2_1",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya2_2",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            }
                        ],
                        "currentEquip": null,
                        "equip": {}
                    },
                    "char_1037_amiya3": {
                        "skinId": "char_1037_amiya3#2",
                        "defaultSkillIndex": 1,
                        "skills": [
                            {
                                "skillId": "skchr_amiya3_1",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            },
                            {
                                "skillId": "skchr_amiya3_2",
                                "unlock": 1,
                                "state": 0,
                                "specializeLevel": 3,
                                "completeUpgradeTime": -1
                            }
                        ],
                        "currentEquip": "uniequip_002_amiya3",
                        "equip": {
                            "uniequip_001_amiya3": {
                                "hide": 0,
                                "locked": 0,
                                "level": 1
                            },
                            "uniequip_002_amiya3": {
                                "hide": 0,
                                "locked": 0,
                                "level": 3
                            }
                        },
                    }
                }
            });
            for equip in get_keys(&equip_table["charEquip"]["char_002_amiya"]) {
                let mut lvl = 1;
                if get_keys(&battleequip_table).contains(&equip) {
                    lvl = battleequip_table[&equip]["phases"].as_array().unwrap().len();
                }
                temp_char_list[&cid]["tmpl"]["char_002_amiya"]["equip"] = json!({
                    equip: {
                        "hide": 0,
                        "locked": 0,
                        "level": lvl
                    }
                });
            }
            temp_char_list[&cid]["tmpl"]["char_002_amiya"]["currentEquip"] = equip_table["charEquip"]["char_002_amiya"].as_array().unwrap().last().unwrap().clone();
        } else if operator_keys[count] == "char_512_aprot" {
            temp_char_list[&cid]["skin"] = json!("char_512_aprot#1");
        }

        building_chars[&cid] = json!({
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

    let mut count_inst_id = 10000usize;

    let dupes = config["charConfig"]["duplicateUnits"].as_array().unwrap().clone();
    for dupe in dupes {
        let dupe = dupe.as_str().unwrap();
        let mut new_char = json!({});
        for char_key in get_keys(&temp_char_list) {
            if temp_char_list[&char_key]["charId"].as_str().unwrap() == dupe {
                new_char = temp_char_list[&char_key].clone();
                break;
            }
        }
        new_char["instId"] = json!(count_inst_id);
        temp_char_list[count_inst_id.to_string()] = new_char;
        count_inst_id += 1;
    }

    player_data["user"]["troop"]["chars"] = temp_char_list;
    player_data["user"]["troop"]["charGroup"] = char_group;
    player_data["user"]["troop"]["curCharInstId"] = json!(count_inst_id);

    // Story
    let mut story_list = json!({"init": 1});
    for story in get_keys(&story_table) {
        story_list[story] = json!(1);
    }

    player_data["user"]["status"]["flags"] = story_list;

    // Stages
    let mut stage_list = json!({});
    for stage in get_keys(&stage_table["stages"]) {
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
    player_data["user"]["dungeon"]["stages"] = json!(stage_list);

    // Addons
    let mut addon_list = json!({});
    let addon_table = update_data(HANDBOOK_INFO_TABLE_URL).await;
    for char_id in get_keys(&addon_table["handbookDict"]) {
        addon_list[&char_id] = json!({"story":{}});
        let story = addon_table["handbookDict"][&char_id]["handbookAvgList"].as_array().unwrap();
        for st in story {
            if st.get("storySetId").is_some() {
                let st_set_id = st["storySetId"].as_str().unwrap();
                addon_list[&char_id]["story"][st_set_id] = json!({
                    "fts": 1649232340,
                    "rts": 1649232340
                });
            }
        }
    }
    for char in get_keys(&addon_table["handbookStageData"]) {
        let stage_id = addon_table["handbookStageData"][&char]["stageId"].as_str().unwrap();
        addon_list[&char]["stage"][&stage_id] = json!({
            "startTimes": 0,
            "completeTimes": 1,
            "state": 3,
            "fts": 1624284657,
            "rts": 1624284657,
            "startTime": 2
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

    // BGs
    let mut bg_list = json!({});
    for bg in display_meta_table["homeBackgroundData"]["homeBgDataList"].as_array().unwrap() {
        let id = bg["bgId"].as_str().unwrap();
        bg_list[&id] = json!({
            "unlock": time(),
        });
    }
    player_data["user"]["background"]["bgs"] = bg_list;

    // Themes
    let mut theme_list = json!({});
    for theme in display_meta_table["homeBackgroundData"]["themeList"].as_array().unwrap() {
        let id = theme["id"].as_str().unwrap();
        theme_list[&id] = json!({
            "unlock": time(),
        });
    }
    player_data["user"]["homeTheme"]["themes"] = theme_list;

    // CHARM
    for charm in charm_table["charmList"].as_array().unwrap() {
        let id = charm["id"].as_str().unwrap();
        player_data["user"]["charm"]["charms"][id] = json!(1);
    }

    // IDEAL CITY CARS
    let car_table = activity_table["carData"]["carDict"].clone();
    for car_gear in get_keys(&car_table) {
        player_data["user"]["car"]["accessories"][&car_gear] = json!({
            "id": &car_gear,
            "num": get_length(&car_table[&car_gear]["posList"])
        })
    }

    let act_table = activity_table["activity"]["TYPE_ACT17SIDE"]["act17side"].clone();
    for place in get_keys(&act_table["placeDataMap"]) {
        player_data["user"]["deepSea"]["places"][&place] = json!(2);
    }
    for node in get_keys(&act_table["nodeInfoDataMap"]) {
        player_data["user"]["deepSea"]["nodes"][&node] = json!(2);
    }
    for choice_node in get_keys(&act_table["choiceNodeDataMap"]) {
        let mut cv = Vec::new();
        for _ in act_table["choiceNodeDataMap"][&choice_node]["optionList"].as_array().unwrap() {
            cv.push(json!(2));
        }
        player_data["user"]["deepSea"]["choices"][&choice_node] = json!(cv);
    }
    for event in get_keys(&act_table["eventDataMap"]) {
        player_data["user"]["deepSea"]["events"][&event] = json!(1);
    }
    for treasure in get_keys(&act_table["treasureNodeDataMap"]) {
        player_data["user"]["deepSea"]["treasures"][&treasure] = json!(1);
    }
    for story in get_keys(&act_table["storyNodeDataMap"]) {
        let story_key = act_table["storyNodeDataMap"][story]["storyKey"].as_str().unwrap();
        player_data["user"]["deepSea"]["stories"][story_key] = json!(1);
    }
    for tech in get_keys(&act_table["techTreeDataMap"]) {
        let branch = &act_table["techTreeDataMap"][&tech]["defaultBranchId"];
        player_data["user"]["deepSea"]["techTrees"][&tech] = json!({
            "state": 2,
            "branch": branch
        });
    }
    for log in get_keys(&act_table["archiveItemUnlockDataMap"]) {
        if !log.starts_with("act17side_log_") {
            continue;
        }
        let chapter = &act_table["archiveItemUnlockDataMap"][&log]["chapterId"].as_str().unwrap();
        if get_keys(&player_data["user"]["deepSea"]["logs"]).iter().map(|s| s.as_str()).collect::<Vec<&str>>().contains(chapter) {
            let sv = player_data["user"]["deepSea"]["logs"][&chapter].as_array_mut().unwrap();
            sv.push(json!(log));
            player_data["user"]["deepSea"]["logs"][&chapter] = json!(sv);
        } else {
            player_data["user"]["deepSea"]["logs"][&chapter] = json!([log]);
        }
    }

    // Mails
    let received_mails = mail_data["recievedIDs"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<u64>>();
    let deleted_mails = mail_data["deletedIDs"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect::<Vec<u64>>();
    for mail_id in get_keys(&mail_data["mailList"]) {
        if received_mails.clone().contains(&mail_id.parse::<u64>().unwrap()) && deleted_mails.clone().contains(&mail_id.parse::<u64>().unwrap()) {
            player_data["user"]["pushFlags"]["hasGifts"] = json!(1);
            break;
        }
    }

    player_data["user"]["status"]["lastRefreshTs"] = json!(time());
    player_data["user"]["status"]["lastApAddTime"] = json!(time());
    player_data["user"]["status"]["registerTs"] = json!(time());
    player_data["user"]["status"]["lastOnlineTs"] = json!(time());
    player_data["user"]["status"]["nickName"] = json!(nick_name);
    player_data["user"]["status"]["nickNumber"] = json!(nick_id);
    player_data["user"]["crisis"]["lst"] = json!(time());
    player_data["user"]["crisis"]["nst"] = json!(time() + 3600);
    player_data["ts"] = json!(time());

    // REPLAY CODES
    let replay_data = read_json(BATTLE_REPLAY_JSON_PATH);

    let cur_char_conf = encrypt(operator_template);

    let force_battle_replay = config["userConfig"]["forceEnableBattleReplay"].as_bool().unwrap();

    if get_keys(&replay_data["saved"]).contains(&cur_char_conf) || force_battle_replay {
        for replay in get_keys(&replay_data["saved"][&cur_char_conf]) {
            if get_keys(&player_data["user"]["dungeon"]["stages"]).contains(&replay) {
                player_data["user"]["dungeon"]["stages"][replay]["hasBattleReplay"] = json!(1);
            }
        }
    }

    let squads = read_json(SQUADS_PATH);

    let mut char_id_map = json!({});

    for char in get_values(&player_data["user"]["troop"]["chars"]) {
        let char_id = char["charId"].as_str().unwrap();
        char_id_map[char_id] = char["instId"].clone();
    }

    let mut squads_data = json!({});
    for id in get_keys(&squads) {
        let mut ct = 0;
        let mut slots = Vec::new();
        for (cntr, slot) in squads[id.to_string()]["slots"].as_array().unwrap().iter().enumerate() {
            let mut slot_data = json!({});
            if cntr >= 12 {
                break;
            }
            let char_id = slot["charId"].as_str().unwrap();
            if char_id_map.get(char_id).is_some() {
                let inst_id = char_id_map[char_id].as_i64().unwrap();
                slot_data["charInstId"] = json!(inst_id);
                let current_equip = slot["currentEquip"].as_str();
                match current_equip {
                    Some(equip) => {
                        if get_keys(&player_data["user"]["troop"]["chars"][inst_id.to_string()]["equip"]).contains(&equip.to_string()) {
                            slot_data["currentEquip"] = json!(equip);
                        } else {
                            slot_data["currentEquip"] = json!(null);
                        }
                    }
                    None => {
                        slot_data["currentEquip"] = json!(null);
                    }
                }
                slot_data["skillIndex"] = slot["skillIndex"].clone();
                slots.push(slot_data);
                ct += 1;
            }
        }
        for _ in ct..12 {
            slots.push(json!(null))
        }
        if slots.len() > 12 {
            slots = slots[..12].to_vec();
        }
        squads_data[id.to_string()]["squadId"] = json!(id.to_string());
        squads_data[id.to_string()]["name"] = squads[id.to_string()]["name"].clone();
        squads_data[id.to_string()]["slots"] = json!(slots);
    }

    let secretary = &config["userConfig"]["secretary"];
    let secretary_skin_id = &config["userConfig"]["secretarySkinId"];
    let bg = &config["userConfig"]["background"];
    let theme = &config["userConfig"]["theme"];

    player_data["user"]["status"]["secretary"] = json!(secretary);
    player_data["user"]["status"]["secretarySkinId"] = json!(secretary_skin_id);
    player_data["user"]["background"]["selected"] = json!(bg);
    player_data["user"]["homeTheme"]["selected"] = json!(theme);

    let tower_ss = &config["towerConfig"]["season"];
    player_data["user"]["tower"]["season"]["id"] = json!(tower_ss);

    let mut story_review_groups = json!({});
    for id in get_keys(&story_review_table) {
        story_review_groups[&id] = json!({
            "rts": 1700000000,
        });
        let mut story_vec = Vec::new();
        for data in story_review_table[&id]["infoUnlockDatas"].as_array().unwrap() {
            story_vec.push(json!({
                "id": data["storyId"],
                "uts": 1695000000,
                "rc": 1
            }));
        }
        story_review_groups[&id]["stories"] = json!(story_vec);
        let mut reward_vec = Vec::new();
        if get_keys(&story_review_meta_table["miniActTrialData"]["miniActTrialDataMap"]).contains(&id) {
            for data in story_review_meta_table["miniActTrialData"]["miniActTrialDataMap"][&id]["rewardList"].as_array().unwrap() {
                reward_vec.push(json!(data["trialRewardId"]))
            }
        }
        story_review_groups[&id]["trailRewards"] = json!(reward_vec);
    }
    player_data["user"]["storyreview"]["groups"] = story_review_groups;

    let mut enemies = json!({});
    for enemy in get_keys(&enemy_handbook_table["enemyData"]) {
        enemies[&enemy] = json!(1);
    }
    player_data["user"]["dexNav"]["enemy"]["enemies"] = enemies;

    for act_id in get_keys(&activity_table["activity"]) {
        if player_data["user"]["activity"].get(&act_id).is_none() {
            player_data["user"]["activity"][&act_id] = json!({});
        }
        for act_data in get_keys(&activity_table["activity"][&act_id]) {
            if player_data["user"]["activity"][&act_id].get(&act_data).is_none() {
                player_data["user"]["activity"][&act_id][&act_data] = json!({});
            }
        }
    }

    player_data["user"]["medal"] = json!({"medals": {}});
    for medal in medal_table["medalList"].as_array().unwrap() {
        let medal_id = medal["medalId"].as_str().unwrap();
        player_data["user"]["medal"]["medals"][medal_id] = json!({
            "id": medal_id,
            "val": [],
            "fts": 1695000000,
            "rts": 1695000000,
        });
    }

    write_json(USER_JSON_PATH, &player_data).unwrap_or(());

    for theme in get_keys(&player_data["user"]["rlv2"]["outer"]) {
        if get_keys(&rlv2_table["details"]).contains(&theme) {
            for stg in get_keys(&rlv2_table["details"][&theme]["stages"]) {
                player_data["user"]["rlv2"]["outer"][&theme]["record"]["stageCnt"][&stg] = json!(1);
            }
        }
    }

    let crisis_v2 = config["crisisV2Config"]["selectedCrisis"].as_str();

    if let Some(ccv2) = crisis_v2 {
        let rune = read_json(format!("{CRISIS_V2_JSON_BASE_PATH}{ccv2}.json"));
        let ss = rune["info"]["seasonId"].as_str().unwrap();
        player_data["user"]["crisisV2"]["current"] = json!(ss);
    }

    let Json(building) = building_sync().await;
    player_data["user"]["building"] = json!(building["playerDataDelta"]["modified"]["building"]);
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
        "yostar_account": "terraps@xcode.com",
        "yostar_token": "a",
        "yostar_uid": "1"
    }))
}
