use serde_json::{json, Value};

use crate::constants::config::MAILLIST_PATH;

use super::json::{get_keys, read_json, write_json};

pub fn get_item(payload: Value, key: &str) -> (Vec<Value>, i32) {
    let mut items = Vec::new();
    let mut has_gift = 1;
    let mut mail_data = read_json(MAILLIST_PATH);
    let mut received_mails = mail_data["recievedIDs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect::<Vec<u64>>();

    if key != "sysMailIdList" {
        let mail = &payload[key];
        received_mails.push(mail.as_str().unwrap().parse().unwrap());
        if get_keys(&mail_data["mailList"][&mail.as_str().unwrap()])
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<&str>>()
            .contains(&"items")
        {
            items.push(mail_data["mailList"][&mail.as_str().unwrap()]["items"].clone())
        }
    } else {
        let mails = payload[key].as_array().unwrap();
        for mail in mails {
            received_mails.push(mail.as_str().unwrap().parse().unwrap());
            if get_keys(&mail_data["mailList"][&mail.as_str().unwrap()])
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .contains(&"items")
            {
                items.push(mail_data["mailList"][&mail.as_str().unwrap()]["items"].clone())
            }
        }
    }

    mail_data["recievedIDs"] = json!(received_mails);

    if received_mails.len() == get_keys(&mail_data["mailList"]).len() {
        has_gift = 0;
    }

    write_json(MAILLIST_PATH, mail_data);

    (items, has_gift)
}
