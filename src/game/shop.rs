use axum::Json;
use serde_json::json;

use crate::utils::json::JSON;

pub async fn pay_get_unconfirmed_order_id_list() -> JSON {
    Json(json!({
        "goodList":[],
        "playerDataDelta":{
            "modified":{},
            "deleted":{}
        }
    }))
}
