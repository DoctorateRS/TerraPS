use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn account_login() -> Json<Value> {
    let uid = Uuid::new_v4().to_string();
    Json(json!(
        {
            "result": 0,
            "uid": uid,
            "secret": "yostar",
            "serviceLicenseVersion": 0
        }
    ))
}

#[tokio::test]
async fn test_account_login() {
    let json_value = account_login().await.0;
    let json_string = serde_json::to_string_pretty(&json_value).unwrap();
    println!("{:#?}", json_string);
}