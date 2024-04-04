use axum::Json;
use serde_json::json;

use crate::{
    constants,
    core::JSON,
    utils::{read_json, write_json},
};

pub async fn background_set_bg(Json(payload): JSON) -> JSON {
    let bg_id = payload["bgId"].as_str().unwrap();
    let mut config = read_json(constants::config::CONFIG_JSON_PATH);
    config["userConfig"]["background"] = bg_id.into();
    write_json(constants::config::CONFIG_JSON_PATH, config);
    let mut user_data = read_json(constants::user::USER_JSON_PATH);
    user_data["user"]["background"]["selected"] = bg_id.into();
    write_json(constants::user::USER_JSON_PATH, user_data);
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
    let mut config = read_json(constants::config::CONFIG_JSON_PATH);
    config["userConfig"]["background"] = theme_id.into();
    write_json(constants::config::CONFIG_JSON_PATH, config);
    let mut user_data = read_json(constants::user::USER_JSON_PATH);
    user_data["user"]["background"]["selected"] = theme_id.into();
    write_json(constants::user::USER_JSON_PATH, user_data);
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
