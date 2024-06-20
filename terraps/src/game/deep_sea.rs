use axum::Json;
use serde_json::json;

use crate::{constants::user::USER_JSON_PATH, utils::json::JSON};
use common_utils::{read_json, write_json};

pub async fn deep_sea_branch(Json(payload): JSON) -> JSON {
    let branches = payload["branches"].clone();
    let mut tech_tree = json!({});

    for branch in branches.as_array().unwrap() {
        tech_tree[branch["techTreeId"].as_str().unwrap()] = json!({
            "branch": branch["branchId"],
            "state": 2
        });
    }

    let mut user_data = read_json(USER_JSON_PATH);
    user_data["user"]["deepSea"]["techTrees"] = tech_tree.clone();
    write_json(USER_JSON_PATH, user_data);

    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {
                "deepSea": {
                    "techTrees": tech_tree
                }
            }
        }
    }))
}

pub async fn deep_sea_event() -> JSON {
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {}
        }
    }))
}
