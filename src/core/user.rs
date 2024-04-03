use axum::Json;
use serde_json::{json, Value};

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
