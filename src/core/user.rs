use axum::Json;
use serde_json::{json, Value};

use crate::{
    constants,
    utils::json::{read_json, write_json, JSON},
};

pub mod app {
    use axum::Json;
    use reqwest::get as reqget;
    use serde_json::{json, Value};

    use crate::utils::json::JSON;

    pub async fn app_v1_config() -> JSON {
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

    pub async fn app_get_settings() -> JSON {
        Json(
            reqget("https://passport.arknights.global/app/getSettings")
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap(),
        )
    }

    pub async fn app_get_code() -> JSON {
        Json(
            reqget("https://passport.arknights.global/app/getCode")
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap(),
        )
    }
}

pub async fn agreement_version() -> JSON {
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

pub async fn info_v1_basic() -> JSON {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "hgId": "1",
            "phone": "doc-rs",
            "email": "doc-rs",
            "identityNum": "doc-rs",
            "identityName": "doc-rs",
            "isMinor": false,
            "isLatestUserAgreement": true
        }
    }))
}

pub async fn user_check_in() -> JSON {
    Json(json!({
        "JSONult": 0,
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn user_agreement() -> JSON {
    Json(json!({
        "data": [
            "lol idk"
        ],
        "version": "4.0.0"
    }))
}

pub async fn auth_v1_token_by_phone_password() -> JSON {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "token": "doc-rs"
        }
    }))
}

pub async fn user_login() -> JSON {
    Json(json!({
        "accessToken": "1",
        "birth": Value::Null,
        "channelId": "",
        "isAuthenticate": true,
        "isLatestUserAgreement": true,
        "isMinor": false,
        "needAuthenticate": false,
        "result": 0,
        "token": "abcd",
        "yostar_username": "doc-rs@gitea.com",
        "yostar_uid": "1",
        "uid": "1"
    }))
}

pub async fn user_v1_get_token() -> JSON {
    let ext = json!({"isMinor": false,"isAuthenticate": true}).to_string();
    Json(json!({
        "channelUid": "1",
        "error": "",
        "extension": ext,
        "isGuest": 0,
        "result": 0,
        "token": "abcd",
        "uid": "1"
    }))
}

pub async fn user_oauth2_v1_grant() -> JSON {
    Json(json!({
        "data": {
            "code": "abcd",
            "uid": "1"
        },
        "msg": "OK",
        "status": 0
    }))
}

pub async fn user_oauth2_v2_grant() -> JSON {
    Json(json!({
        "status": 0,
        "msg": "OK",
        "data": {
            "code": "doc-rs",
            "uid": "1"
        }
    }))
}

pub async fn user_yostar_create_login() -> JSON {
    Json(json!({
        "isNew": 0,
        "result": 0,
        "token": "1",
        "uid": "1",
        "yostar_uid": "1",
        "yostar_username": "Doctorate@doctorate.com"
    }))
}

pub async fn user_need_cloud_auth() -> JSON {
    Json(json!({
        "msg": "OK",
        "status": 0
    }))
}

pub async fn user_auth() -> JSON {
    Json(json!({
        "isAuthenticate": true,
        "isGuest": false,
        "isLatestUserAgreement": true,
        "isMinor": false,
        "needAuthenticate": false,
        "uid": "1"
    }))
}

pub async fn user_change_resume(Json(payload): JSON) -> JSON {
    let resume = payload["resume"].clone();
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "status": {
                    "resume": resume
                }
            },
            "deleted": {}
        }
    }))
}

pub async fn user_change_secretary(Json(payload): JSON) -> JSON {
    let mut config = read_json(constants::config::CONFIG_JSON_PATH).clone();
    let mut user_data = read_json(constants::user::USER_JSON_PATH).clone();
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

pub async fn user_change_avatar(Json(payload): JSON) -> JSON {
    let avatar = payload;
    let mut user_data = read_json(constants::user::USER_JSON_PATH).clone();
    user_data["user"]["status"]["avatarId"] = avatar.clone();
    write_json(constants::user::USER_JSON_PATH, user_data);
    Json(json!({
        "playerDataDelta": {
            "modified": {
                "status": {
                    "avatarId": avatar
                }
            },
            "deleted": {}
        }
    }))
}

pub mod business_card {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

    pub async fn change_name_component(Json(payload): JSON) -> JSON {
        let component = payload["component"].clone();
        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "nameCardStyle": {
                        "componentOrder": component
                    }
                },
                "deleted": {}
            }
        }))
    }

    pub async fn change_card_skin(Json(payload): JSON) -> JSON {
        let skin_id = payload["skinId"].clone();
        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "nameCardStyle": {
                        "skin": {
                            "selected": skin_id
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }
}
