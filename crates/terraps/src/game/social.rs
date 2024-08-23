use crate::utils::json::JSON;
use axum::Json;
use common_utils::write_json;
use serde_json::json;

pub async fn social_search_player(Json(payload): JSON) -> JSON {
    let username = payload["idList"].as_array().unwrap()[0].clone();
    Json(json!({
        "players": [
            {
                "nickName": username,
                "nickNumber": "6666",
                "uid": "66666666",
                "friendNumLimit": 50,
                "serverName": "泰拉",
                "level": 120,
                "avatarId": "0",
                "avatar": {},
                "assistCharList": [
                    null
                ],
                "lastOnlineTime": 0,
                "medalBoard": {
                    "type": "EMPTY",
                    "custom": null,
                    "template": null
                },
                "skin": {
                    "selected": "nc_rhodes_default",
                    "state": {}
                }
            }
        ],
        "friendStatusList": [
            0
        ],
        "resultIdList": [
            "66666666"
        ],
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn social_get_sort_list_info() -> JSON {
    Json(json!({
        "result": [],
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn social_set_assist_char_list(Json(payload): JSON) -> JSON {
    write_json("./dump/social_assist.json", &payload).unwrap();
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "social": payload
            },
            "deleted": {}
        }
    }))
}

pub async fn social_set_card_medal(Json(payload): JSON) -> JSON {
    let data = payload;
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "social": {
                    "medalBoard": {
                        "type": data["type"],
                        "template": data["templateGroup"]
                    }
                }
            },
            "deleted": {}
        }
    }))
}
