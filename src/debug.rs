// DEBUGGING FUNCTIONS HERE

use axum::Json;
use serde_json::json;

use crate::utils::json::JSON;

pub async fn decrypt_battle_data(Json(payload): JSON) -> JSON {
    Json(json!({}))
}
