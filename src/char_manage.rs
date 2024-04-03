pub mod char {
    use crate::{constants::user::USER_JSON_PATH, core::JSON, utils::read_json};
    use axum::Json;
    use serde_json::json;

    pub async fn char_change_mark_star(Json(payload): JSON) -> JSON {
        let set = payload["set"].clone();

        let data = json!({
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
        let mut chars = user_data["user"]["troop"]["chars"].clone();
        Json(data)
    }
}

pub mod char_build {}
