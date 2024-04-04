pub mod char {
    use crate::{
        constants::user::USER_JSON_PATH,
        core::JSON,
        utils::{read_json, write_json},
    };
    use axum::Json;
    use serde_json::json;

    pub async fn char_change_mark_star(Json(payload): JSON) -> JSON {
        let set = payload["set"].clone();

        let mut data = json!({
            "playerDataDelta": {
                "deleted": {},
                "modified": {
                    "troop": {
                        "chars": {
                        }
                    }
                }
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);
        let chars = user_data["user"]["troop"]["chars"].clone();
        let mut index_list = Vec::new();
        for (character, _) in set.as_object().unwrap() {
            for (char_index, saved_char) in chars.as_object().unwrap() {
                if saved_char["charId"].as_str().unwrap() == character {
                    index_list.push(char_index);
                }
            }
            for index in index_list.iter() {
                user_data["user"]["troop"]["chars"][index]["starMark"] = set[character].clone();
                data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
                    *index: {
                        "starMark": set[character].clone()
                    }
                })
            }
        }

        write_json(USER_JSON_PATH, user_data);
        Json(data)
    }
}

pub mod char_build {
    use crate::{
        constants::user::USER_JSON_PATH,
        core::{time, JSON},
        utils::{read_json, write_json},
    };
    use axum::Json;
    use serde_json::json;

    pub async fn char_build_batch_set_char_voice_lan() -> JSON {
        Json(json!({
            "result": {},
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn char_build_addon_story_unlock(Json(payload): JSON) -> JSON {
        let story_id = payload["storyId"].as_str().unwrap();
        let char_id = payload["charId"].as_str().unwrap();
        let ts = json!({
            "fts": time(),
            "rts": time()
        });
        let mut data = json!({
            "playerDataDelta": {
                "deleted": {},
                "modified": {
                    "troop": {
                        "addon": {
                        }
                    }
                }
            }
        });
        let char_data = json!({
            char_id: {
                "story": {
                    story_id: ts
                }
            }
        });
        let mut user_data = read_json(USER_JSON_PATH);
        user_data["user"]["troop"]["addon"] = char_data.clone();
        data["playerDataDelta"]["modified"]["troop"]["addon"] = char_data;
        write_json(USER_JSON_PATH, user_data);
        Json(data)
    }

    pub async fn char_build_set_char_voice_lan(Json(payload): JSON) -> JSON {
        let char_list = payload["charList"].clone();
        let voice_lan = payload["voiceLan"].clone();

        let mut data = json!({
            "playerDataDelta": {
                "deleted": {},
                "modified": {
                    "troop": {
                        "chars": {
                        }
                    }
                }
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);

        for (character, _) in char_list.as_object().unwrap() {
            user_data["user"]["troop"]["chars"][character]["voiceLan"] = voice_lan.clone();
            data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
                character: {
                    "voiceLan": voice_lan
                }
            })
        }

        Json(data)
    }

    pub async fn char_build_set_char_default_skill(Json(payload): JSON) -> JSON {
        let char_inst_id = payload["charInstId"].clone();
        let char_inst_id = if char_inst_id.is_string() {
            char_inst_id.as_str().unwrap().to_string()
        } else {
            char_inst_id.as_u64().unwrap().to_string()
        };

        let default_skill_index = payload["defaultSkillIndex"].clone();

        let mut data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "chars":{}
                    }
                },
                "deleted":{}
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);

        data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
            &char_inst_id: {
                "defaultSkillIndex": default_skill_index
            }
        });

        user_data["user"]["troop"]["chars"][char_inst_id]["defaultSkillIndex"] = default_skill_index;
        write_json(USER_JSON_PATH, user_data);

        Json(data)
    }

    pub async fn char_build_change_char_skin(Json(payload): JSON) -> JSON {
        let char_inst_id = payload["charInstId"].clone();
        let char_inst_id = if char_inst_id.is_string() {
            char_inst_id.as_str().unwrap().to_string()
        } else {
            char_inst_id.as_u64().unwrap().to_string()
        };
        let skin_id = payload["skinId"].clone();

        let mut data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "chars":{}
                    }
                },
                "deleted":{}
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);

        data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
            &char_inst_id: {
                "skinId": skin_id
            }
        });

        user_data["user"]["troop"]["chars"][char_inst_id]["skinId"] = skin_id;
        write_json(USER_JSON_PATH, user_data);

        Json(data)
    }

    pub async fn char_build_set_char_equipment(Json(payload): JSON) -> JSON {
        let char_inst_id = payload["charInstId"].clone();
        let char_inst_id = if char_inst_id.is_string() {
            char_inst_id.as_str().unwrap().to_string()
        } else {
            char_inst_id.as_u64().unwrap().to_string()
        };
        let equip_id = payload["equipId"].clone();

        let mut data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "chars":{}
                    }
                },
                "deleted":{}
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);

        data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
            &char_inst_id: {
                "currentEquip": equip_id
            }
        });

        user_data["user"]["troop"]["chars"][char_inst_id]["currentEquip"] = equip_id;
        write_json(USER_JSON_PATH, user_data);

        Json(data)
    }

    pub async fn char_build_change_char_template(Json(payload): JSON) -> JSON {
        let char_inst_id = payload["charInstId"].clone();
        let char_inst_id = if char_inst_id.is_string() {
            char_inst_id.as_str().unwrap().to_string()
        } else {
            char_inst_id.as_u64().unwrap().to_string()
        };
        let template_id = payload["templateId"].clone();

        let data = json!({
            "playerDataDelta":{
                "modified":{
                    "troop":{
                        "chars":{
                            &char_inst_id: {
                                "currentTmpl": template_id
                            }
                        }
                    }
                },
                "deleted":{}
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);
        user_data["user"]["troop"]["chars"][char_inst_id]["currentTmpl"] = template_id;

        write_json(USER_JSON_PATH, user_data);
        Json(data)
    }
}

pub mod charm {
    use axum::Json;
    use serde_json::json;

    use crate::{
        constants::user::USER_JSON_PATH,
        core::JSON,
        utils::{read_json, write_json},
    };

    pub async fn charm_set_squad(Json(payload): JSON) -> JSON {
        let charm_squad = payload["squad"].clone();
        let mut user_data = read_json(USER_JSON_PATH);
        user_data["user"]["charm"]["squad"] = charm_squad.clone();
        write_json(USER_JSON_PATH, user_data);

        Json(json!({
            "playerDataDelta": {
                "deleted": {},
                "modified": {
                    "charm": {
                        "squad": charm_squad
                    }
                }
            }
        }))
    }
}
