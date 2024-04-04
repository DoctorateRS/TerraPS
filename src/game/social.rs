use crate::core::JSON;
use axum::Json;
use serde_json::json;

pub async fn social_set_assist_char_list(Json(payload): JSON) -> JSON {
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
