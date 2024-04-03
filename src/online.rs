use crate::core::JSON;
use axum::Json;
use serde_json::json;

pub async fn online_v1_ping() -> JSON {
    Json(json!({
        "alertTime": 600,
        "interval": 3590,
        "message": "OK",
        "result": 0,
        "timeLeft": -1
    }))
}

pub async fn online_v1_login_out() -> JSON {
    Json(json!({
        "error": "Not Found",
        "message": "Not Found",
        "statusCode": 404
    }))
}
