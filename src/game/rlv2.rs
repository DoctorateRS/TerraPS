use axum::Json;
use serde_json::{json, Value};

use crate::utils::json::JSON;

pub async fn rlv2_give_up_game() -> JSON {
    Json(json!({
            "result": "ok",
            "playerDataDelta": {
                "modified": {
                    "rlv2": {
                        "current": {
                            "player": Value::Null,
                            "record": Value::Null,
                            "map": Value::Null,
                            "troop": Value::Null,
                            "inventory": Value::Null,
                            "game": Value::Null,
                            "buff": Value::Null,
                            "module": Value::Null
                        }
                    }
                },
                "deleted": {}
            }
        }
    ))
}
