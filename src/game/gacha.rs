pub mod normal {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

    pub async fn sync_normal_gacha() -> JSON {
        Json(json!({
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }))
    }
}

pub mod advanced {}
