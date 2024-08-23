use axum::Json;
use serde_json::json;

use crate::{
    constants::config::MAILLIST_PATH,
    core::time,
    utils::{
        json::{get_keys, JSON},
        mail::get_item,
    },
};

use common_utils::{read_json, write_json};

pub async fn mail_get_meta_info_list() -> JSON {
    let mut result = Vec::new();
    let mail_data = read_json(MAILLIST_PATH);
    let received_ids = &mail_data["recievedIDs"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect::<Vec<u64>>();
    let deleted_ids = &mail_data["deletedIds"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect::<Vec<u64>>();
    for mail in get_keys(&mail_data["mailList"]) {
        if deleted_ids.contains(&mail.parse().unwrap()) {
            continue;
        } else {
            let state = if received_ids.contains(&mail.parse().unwrap()) { 1 } else { 0 };
            let mail_data = json!({
                "createAt": time(),
                "hasItem": 1,
                "mailId": &mail,
                "state": state,
                "type": 1
            });
            result.push(mail_data);
        }
    }
    Json(json!({
        "result": result,
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}

pub async fn mail_list_mail_box() -> JSON {
    let mut result = Vec::new();
    let mail_data = read_json(MAILLIST_PATH);
    let received_ids = &mail_data["recievedIDs"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect::<Vec<u64>>();
    let deleted_ids = &mail_data["deletedIds"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect::<Vec<u64>>();
    let mut has_gift = 0;
    for mail in get_keys(&mail_data["mailList"]) {
        if deleted_ids.contains(&mail.parse().unwrap()) {
            continue;
        }
        if received_ids.contains(&mail.parse().unwrap()) {
            has_gift = 1;
        }
        let state = if received_ids.contains(&mail.parse().unwrap()) { 1 } else { 0 };
        let mail_data = json!({
            "createAt": time(),
            "expireAt": time() + 31536000,
            "mailId": &mail,
            "platform": -1,
            "state": state,
            "style": {},
            "type": 1,
            "uid": ""
        });
        result.push(json!({&mail: mail_data}));
    }
    Json(json!({
        "mailList": result,
        "playerDataDelta": {
            "modified": {
                "pushFlags": {
                    "hasGifts": has_gift
                }
            },
            "deleted": {}
        }
    }))
}

pub async fn mail_receive_mail(Json(payload): JSON) -> JSON {
    let (result, has_gift) = get_item(payload, "mailId");
    Json(json!({
        "items": result,
        "playerDataDelta": {
            "modified": {
                "consumable": {},
                "inventory":{},
                "pushFlags": {
                    "hasGifts": has_gift as i8
                },
                "status": {}
            },
            "deleted": {}
        }
    }))
}

pub async fn mail_receive_all_mail(Json(payload): JSON) -> JSON {
    let (result, _) = get_item(payload, "sysMailIdList");
    Json(json!({
        "items": result,
        "playerDataDelta": {
            "modified": {
                "consumable": {},
                "inventory":{},
                "pushFlags": {
                    "hasGifts": 0
                },
                "status": {}
            },
            "deleted": {}
        }
    }))
}

pub async fn mail_delete_all_received_mails(Json(payload): JSON) -> JSON {
    let mut mail_data = read_json(MAILLIST_PATH);
    let mut deleted_ids = mail_data["deletedIds"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect::<Vec<u64>>();

    for mail in payload["sysMailIdList"].as_array().unwrap() {
        if !deleted_ids.contains(&mail.as_str().unwrap().parse().unwrap()) {
            deleted_ids.push(mail.as_str().unwrap().parse().unwrap());
        }
    }

    mail_data["deletedIds"] = json!(deleted_ids);
    write_json(MAILLIST_PATH, mail_data).unwrap_or(());

    Json(json!({
        "result": {},
        "playerDataDelta": {
            "modified": {},
            "deleted": {}
        }
    }))
}
