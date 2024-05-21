use axum::Json;
use serde_json::json;

use crate::{constants::user::BATTLE_REPLAY_JSON_PATH, utils::json::JSON};
use common_utils::{read_json, write_json};

pub async fn campaignv2_battle_start(Json(payload): JSON) -> JSON {
    let stage_id = payload["stageId"].as_str().unwrap();

    let mut replay_data = read_json(BATTLE_REPLAY_JSON_PATH);
    replay_data["current"] = json!(stage_id);
    write_json(BATTLE_REPLAY_JSON_PATH, replay_data);

    Json(json!({
        "battleId": "abcdefgh-1234-5678-a1b2c3d4e5f6",
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        },
        "result": 0
    }))
}

pub async fn campaignv2_battle_finish() -> JSON {
    Json(json!({
        "result": 0,
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn campaignv2_battle_sweep() -> JSON {
    Json(json!({
        "result": 0,
        "apFailReturn": 1,
        "rewards": [],
        "unlockStages": [],
        "unusualRewards": [],
        "additionalRewards": [],
        "furnitureRewards": [],
        "diamondMaterialRewards": [
            {
                "type": "DIAMOND_SHD",
                "id": "4003",
                "count": 1
            }
        ],
        "currentFeeBefore": 0,
        "currentFeeAfter": 1,
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}
