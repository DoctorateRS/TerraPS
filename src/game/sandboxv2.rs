use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants::{sandbox::SANDBOX_JSON_PATH, templates::SANDBOX_TEMPLATE},
    utils::json::{read_json, write_json, JSON},
};

pub async fn create_game() -> JSON {
    let sandbox = read_json(SANDBOX_TEMPLATE);
    write_json(SANDBOX_JSON_PATH, &sandbox);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox
            },
            "deleted": {}
        }
    }))
}

pub async fn set_squad(Json(payload): JSON) -> JSON {
    let mut sandbox_data = read_json(SANDBOX_JSON_PATH);

    let index = match &payload["index"] {
        Value::Number(index) => index.as_u64().unwrap().to_string(),
        Value::String(index) => index.as_str().to_string(),
        _ => panic!("Invalid index"),
    };

    sandbox_data["template"]["SANDBOX_V2"]["sandbox_1"]["troop"]["squad"][&index] = json!({
        "slots": payload["slots"],
        "tools": payload["tools"]
    });

    write_json(SANDBOX_JSON_PATH, &sandbox_data);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "sandboxPerm": sandbox_data
            },
            "deleted": {}
        }
    }))
}
