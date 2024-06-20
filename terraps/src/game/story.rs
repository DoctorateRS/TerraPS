use axum::Json;
use serde_json::json;

use crate::utils::json::JSON;

pub async fn story_finish_story() -> JSON {
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {}
        }
    }))
}
