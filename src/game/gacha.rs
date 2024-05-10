pub mod normal {
    use axum::Json;
    use serde_json::{json, Value};

    use crate::{
        constants::user::USER_GACHA_PATH,
        core::time,
        utils::json::{read_json, JSON},
    };

    pub async fn sync_normal_gacha() -> JSON {
        Json(json!({
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }

    pub async fn normal_gacha(Json(payload): JSON) -> JSON {
        let slot_id: &str;
        let num;
        let v = vec![];
        let time = time();
        match &payload["slotId"] {
            Value::String(id) => slot_id = id,
            Value::Number(id) => {
                let n = id.to_string();
                num = n;
                slot_id = &num;
            }
            _ => slot_id = "0",
        };
        let tag_list = payload["tagList"].as_array().unwrap_or(&v);
        let mut tags = vec![];

        for tag in tag_list {
            tags.push(json!({"tagId": tag, "pick": 1}))
        }

        let finish = time + payload["duration"].as_u64().unwrap_or(0);

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "recruit": {
                        "normal": {
                            "slots": {
                                slot_id: {
                                    "state": 2,
                                    "selectTags": tags,
                                    "startTs": time,
                                    "durationInSec": payload["duration"],
                                    "maxFinishTs": finish,
                                    "realFinishTs": finish,
                                }
                            }
                        }
                    }
                },
                "deleted": {},
            }
        }))
    }

    pub async fn boost_normal_gacha(Json(payload): JSON) -> JSON {
        let time = time();
        let slot_id: &str;
        let num;
        match &payload["slotId"] {
            Value::String(id) => slot_id = id,
            Value::Number(id) => {
                let n = id.to_string();
                num = n;
                slot_id = &num;
            }
            _ => slot_id = "0",
        };
        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "recruit": {
                        "normal": {
                            "slots": {
                                slot_id: {
                                    "state": 3,
                                    "realFinishTs": time,
                                }
                            }
                        }
                    }
                },
                "deleted": {},
            }
        }))
    }

    pub async fn finish_normal_gacha(Json(payload): JSON) -> JSON {
        let gacha = read_json(USER_GACHA_PATH);
        let slot_id: &str;
        let num;
        match &payload["slotId"] {
            Value::String(id) => slot_id = id,
            Value::Number(id) => {
                let n = id.to_string();
                num = n;
                slot_id = &num;
            }
            _ => slot_id = "0",
        };
        let char_id = gacha["normal"]["charId"].as_str().unwrap();
        let char_inst_id = char_id.split('_').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let is_new = gacha["normal"]["isNew"].as_i64().unwrap();
        let char_id = format!("p_{}", char_id);
        Json(json!({
            "result": 0,
            "charGet": {
                "charInstId": char_inst_id,
                "charId": char_id,
                "isNew": is_new,
                "itemGet": [
                    {"type": "HGG_SHD", "id": "4004", "count": 999},
                    {"type": "LGG_SHD", "id": "4005", "count": 999},
                    {"type": "MATERIAL", "id": char_id, "count": 999},
                ],
                "logInfo": {},
            },
            "playerDataDelta": {
                "modified": {
                    "recruit": {
                        "normal": {
                            "slots": {
                                slot_id: {
                                    "state": 1,
                                    "selectTags": [],
                                    "startTs": -1,
                                    "durationInSec": -1,
                                    "maxFinishTs": -1,
                                    "realFinishTs": -1,
                                }
                            }
                        }
                    }
                },
                "deleted": {},
            },
        }))
    }
}

pub mod advanced {
    use axum::Json;
    use rand::prelude::*;
    use serde_json::json;

    use crate::{
        constants::user::{GACHA_TEMPLATE_JSON_PATH, USER_GACHA_PATH},
        utils::{
            json::{read_json, JSON},
            TRng,
        },
    };

    pub async fn get_pool_detail() -> JSON {
        Json(read_json(GACHA_TEMPLATE_JSON_PATH))
    }

    pub async fn advanced_gacha() -> JSON {
        let gacha = read_json(USER_GACHA_PATH);
        let pool_len = gacha["advanced"].as_array().unwrap().len();

        let mut gacha_rng = TRng::new();
        let res = gacha_rng.0.gen_range(0..pool_len);

        let gacha_res = gacha["advanced"][res].clone();
        let char_id = gacha_res["charId"].as_str().unwrap();
        let is_new = gacha_res["isNew"].as_i64().unwrap();
        let char_inst_id = char_id.split('_').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

        Json(json!({
            "result": 0,
            "charGet": {
                "charInstId": char_inst_id,
                "charId": char_id,
                "isNew": is_new,
                "itemGet": [
                    {"type": "HGG_SHD", "id": "4004", "count": 999},
                    {"type": "LGG_SHD", "id": "4005", "count": 999},
                    {"type": "MATERIAL", "id": format!("p_{char_id}"), "count": 999},
                ],
                "logInfo": {},
            },
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            },
        }))
    }

    pub async fn ten_advanced_gacha() -> JSON {
        let gacha = read_json(USER_GACHA_PATH);
        let pool_len = gacha["advanced"].as_array().unwrap().len();

        let mut gacha_rng = TRng::new();
        let mut gacha_res_vec = vec![];

        for _ in 0..10 {
            let res = gacha_rng.0.gen_range(0..pool_len);
            let gacha_res = gacha["advanced"][res].clone();
            let char_id = gacha_res["charId"].as_str().unwrap();
            let is_new = gacha_res["isNew"].as_i64().unwrap();
            let char_inst_id = char_id.split('_').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

            gacha_res_vec.push(json!({
                "charInstId": char_inst_id,
                "charId": char_id,
                "isNew": is_new,
                "itemGet": [
                    {"type": "HGG_SHD", "id": "4004", "count": 999},
                    {"type": "LGG_SHD", "id": "4005", "count": 999},
                    {"type": "MATERIAL", "id": format!("p_{char_id}"), "count": 999},
                ],
                "logInfo": {},
            }))
        }

        Json(json!({
            "result": 0,
            "gachaResultList": gacha_res_vec,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            },
        }))
    }
}
