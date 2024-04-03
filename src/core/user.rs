use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants,
    utils::{read_json, write_json},
};

pub async fn app_v1_config() -> Json<Value> {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "antiAddiction": {
                "minorPeriodEnd": 21,
                "minorPeriodStart": 20
            },
            "payment": [
                {
                    "key": "alipay",
                    "recommend": true
                },
                {
                    "key": "wechat",
                    "recommend": false
                },
                {
                    "key": "pcredit",
                    "recommend": false
                }
            ],
            "customerServiceUrl": "https://chat.hypergryph.com/chat/h5/v2/index.html",
            "cancelDeactivateUrl": "https://user.hypergryph.com/cancellation",
            "agreementUrl": {
                "game": "https://user.hypergryph.com/protocol/plain/ak/index",
                "unbind": "https://user.hypergryph.com/protocol/plain/ak/cancellation",
                "account": "https://user.hypergryph.com/protocol/plain/index",
                "privacy": "https://user.hypergryph.com/protocol/plain/privacy",
                "register": "https://user.hypergryph.com/protocol/plain/registration",
                "updateOverview": "https://user.hypergryph.com/protocol/plain/overview_of_changes",
                "childrenPrivacy": "https://user.hypergryph.com/protocol/plain/children_privacy"
            },
            "app": {
                "enablePayment": true,
                "enableAutoLogin": false,
                "enableAuthenticate": true,
                "enableAntiAddiction": true,
                "wechatAppId": "",
                "alipayAppId": "",
                "oneLoginAppId": "",
                "enablePaidApp": false,
                "appName": "明日方舟",
                "appAmount": 600
            }
        }
    }))
}

pub async fn agreement_version() -> Json<Value> {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "agreementUrl": {
                "privacy": "https://user.hypergryph.com/protocol/plain/ak/privacy",
                "service": "https://user.hypergryph.com/protocol/plain/ak/service",
                "updateOverview": "https://user.hypergryph.com/protocol/plain/ak/overview_of_changes",
                "childrenPrivacy": "https://user.hypergryph.com/protocol/plain/ak/children_privacy"
            },
            "authorized": true,
            "isLatestUserAgreement": true
        }
    }))
}

pub async fn info_v1_basic() -> Json<Value> {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "hgId": "1",
            "phone": "doctorate",
            "email": "doctorate",
            "identityNum": "doctorate",
            "identityName": "doctorate",
            "isMinor": false,
            "isLatestUserAgreement": true
        }
    }))
}

pub async fn user_check_in() -> Json<Value> {
    Json(json!({
        "result": 0,
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn user_agreement() -> Json<Value> {
    Json(json!({
        "data": [
            "lol idk"
        ],
        "version": "4.0.0"
    }))
}

pub async fn auth_v1_token_by_phone_password() -> Json<Value> {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "token": "doctorate"
        }
    }))
}

pub async fn user_change_secretary(Json(payload): Json<Value>) -> Json<Value> {
    let mut config = read_json(constants::config::CONFIG_JSON_PATH).clone();
    let mut user_data = read_json(constants::user::USER_JSON_PATH).clone();
    let _ = payload["charInstId"].as_str().unwrap();
    let skin_id = payload["skinId"].as_str().unwrap();
    let secretary = match skin_id.find('@') {
        Some(_) => skin_id.split('@').collect::<Vec<&str>>()[0],
        None => skin_id.split('#').collect::<Vec<&str>>()[0],
    };
    config["userConfig"]["secretary"] = Value::String(secretary.to_string());
    config["userConfig"]["secretarySkinId"] = Value::String(skin_id.to_string());
    user_data["user"]["status"]["secretarySkinId"] = Value::String(skin_id.to_string());
    user_data["user"]["status"]["secretary"] = Value::String(secretary.to_string());
    write_json(constants::config::CONFIG_JSON_PATH, config);
    write_json(constants::user::USER_JSON_PATH, user_data);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "status": {
                    "secretary": secretary,
                    "secretarySkinId": skin_id,
                }
            },
            "deleted": {}
        }
    }))
}
