pub mod normal {
    use axum::Json;
    use serde_json::{json, Value};

    use crate::{core::time, utils::json::JSON};

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

    pub async fn boost_normal_gacha() {}
}

pub mod advanced {}
