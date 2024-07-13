use crate::{
    constants::{config::CONFIG_JSON_PATH, user::USER_JSON_PATH},
    utils::json::JSON,
};
use axum::Json;
use common_utils::{read_json, write_json};
use serde_json::json;

pub async fn background_set_bg(Json(payload): JSON) -> JSON {
    let bg_id = payload["bgId"].clone();
    let mut config = read_json(CONFIG_JSON_PATH);
    config["userConfig"]["background"] = bg_id.clone();
    write_json(CONFIG_JSON_PATH, config).unwrap_or(());
    let mut user_data = read_json(USER_JSON_PATH);
    user_data["user"]["background"]["selected"] = bg_id.clone();
    write_json(USER_JSON_PATH, user_data).unwrap_or(());
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {
                "background": {
                    "selected": bg_id
                }
            }
        }
    }))
}

pub async fn home_theme_change(Json(payload): JSON) -> JSON {
    let theme_id = payload["themeId"].as_str().unwrap();
    let mut config = read_json(CONFIG_JSON_PATH);
    config["userConfig"]["background"] = theme_id.into();
    write_json(CONFIG_JSON_PATH, config).unwrap_or(());
    let mut user_data = read_json(USER_JSON_PATH);
    user_data["user"]["background"]["selected"] = theme_id.into();
    write_json(USER_JSON_PATH, user_data).unwrap_or(());
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
