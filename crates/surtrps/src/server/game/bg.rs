use axum::Json;
use common_utils::{read_json, write_json};
use serde_json::{json, Value};

use crate::cnst::{config::CONFIG_JSON_PATH, user::USER_JSON_PATH};

use models::payload::{BgPayload, ThemePayload};

pub async fn change_bg(Json(payload): Json<BgPayload>) -> Json<Value> {
    let id = payload.bg_id;

    // UPDATE USER CONFIG
    let mut cfg = read_json(CONFIG_JSON_PATH);
    cfg["userConfig"]["background"] = Value::String(id.clone());
    let _ = write_json(CONFIG_JSON_PATH, cfg);

    // UPDATE USER DATA
    let mut data = read_json(USER_JSON_PATH);
    data["user"]["homeTheme"]["selected"] = Value::String(id.clone());
    let _ = write_json(USER_JSON_PATH, data);

    // TODO: REWRITE THIS PILE OF CRAP
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {
                "background": {
                    "selected": id
                }
            }
        }
    }))
}

pub async fn change_theme(Json(payload): Json<ThemePayload>) -> Json<Value> {
    let theme_id = payload.theme_id;

    let mut config = read_json(CONFIG_JSON_PATH);
    config["userConfig"]["theme"] = Value::String(theme_id.clone());
    let _ = write_json(CONFIG_JSON_PATH, config);

    let mut user_data = read_json(USER_JSON_PATH);
    user_data["user"]["background"]["selected"] = Value::String(theme_id.clone());
    let _ = write_json(USER_JSON_PATH, user_data);

    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {
                "homeTheme": {
                    "selected": theme_id
                }
            }
        }
    }))
}
