// DEBUGGING FUNCTIONS HERE

use axum::Json;
use serde_json::json;

use crate::utils::{
    crypto::{hex::from_hex, md5::md5_digest},
    json::JSON,
};

pub async fn decrypt_battle_data(Json(payload): JSON) -> JSON {
    let login_time = payload["login_time"].as_u64();
    let data = payload["data"].as_str().unwrap();

    const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

    let login_time = login_time.unwrap_or(1672502400);
    let ptr = &data.len() - 32;
    let iv = &data[ptr..];
    let data = &data[..ptr];

    let src = format!("{LOG_TOKEN_KEY}{login_time}");
    let key = md5_digest(src.as_bytes());

    let battle_data = match from_hex(data) {
        Ok(data) => match String::from_utf8(data) {
            Ok(data) => data,
            Err(e) => panic!("Error parsing UTF-8: {e}"),
        },
        Err(e) => panic!("Error parsing Integer:{}", e),
    };

    Json(json!({}))
}
