use crate::core::JSON;
use axum::{http::HeaderMap, Json};
use serde_json::json;
use uuid::Uuid;

pub async fn account_login(header: HeaderMap) -> JSON {
    let fallback_uid = Uuid::new_v4().to_string();
    let uid = match header.get("Uid") {
        Some(uid) => uid.to_str().unwrap_or(fallback_uid.as_str()),
        None => fallback_uid.as_str(),
    };
    Json(json!({
        "result": 0,
        "uid": uid,
        "secret": "yostar",
        "serviceLicenseVersion": 0
    }))
}
