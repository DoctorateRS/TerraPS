// DEBUGGING FUNCTIONS HERE

use std::fs::{read, read_to_string};

use axum::Json;
use serde_json::json;

use crate::utils::{
    crypto::{hex::from_hex, md5::md5_digest},
    json::JSON,
};

pub async fn decrypt_battle_data() -> JSON {
    let data = read_to_string("./encrypted/DATA_1.txt").unwrap();

    const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

    let login_time = 1672502400;
    let ptr = &data.len() - 32;
    let iv = &data[ptr..];
    let data = &data[..ptr];

    let src = format!("{LOG_TOKEN_KEY}{login_time}");
    let key = md5_digest(src.as_bytes());

    let battle_data = match from_hex(data) {
        Ok(data) => data,
        Err(e) => panic!("Error parsing Integer:{}", e),
    };

    let iv = match from_hex(iv) {
        Ok(data) => data,
        Err(e) => panic!("Error parsing Integer:{}", e),
    };

    Json(json!({}))
}
