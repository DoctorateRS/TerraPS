pub mod char {
    use crate::{
        constants::user::USER_JSON_PATH,
        core::JSON,
        utils::{read_json, write_json},
    };
    use axum::Json;
    use serde_json::json;

    pub async fn char_change_mark_star(Json(payload): JSON) -> JSON {
        let set = payload["set"].clone();

        let mut data = json!({
            "playerDataDelta": {
                "deleted": {},
                "modified": {
                    "troop": {
                        "chars": {
                        }
                    }
                }
            }
        });

        let mut user_data = read_json(USER_JSON_PATH);
        let chars = user_data["user"]["troop"]["chars"].clone();
        let mut index_list = Vec::new();
        for (character, _) in set.as_object().unwrap() {
            for (char_index, saved_char) in chars.as_object().unwrap() {
                if saved_char["charId"].as_str().unwrap() == character {
                    index_list.push(char_index);
                }
            }
            for index in index_list.iter() {
                user_data["user"]["troop"]["chars"][index]["starMark"] = set[character].clone();
                data["playerDataDelta"]["modified"]["troop"]["chars"] = json!({
                    *index: {
                        "starMark": set[character].clone()
                    }
                })
            }
        }

        write_json(USER_JSON_PATH, user_data);
        Json(data)
    }
}

pub mod char_build {}
