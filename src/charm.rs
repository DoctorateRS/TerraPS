use axum::Json;
use serde_json::json;

use crate::{
    constants::user::USER_JSON_PATH,
    core::JSON,
    utils::{read_json, write_json},
};

pub async fn charm_set_squad(Json(payload): JSON) -> JSON {
    let charm_squad = payload["squad"].clone();
    let mut user_data = read_json(USER_JSON_PATH);
    user_data["user"]["charm"]["squad"] = charm_squad.clone();
    write_json(USER_JSON_PATH, user_data);

    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {
                "charm": {
                    "squad": charm_squad
                }
            }
        }
    }))
}
