pub mod account;
pub mod background;
pub mod building;
pub mod char_manager;
pub mod crisis_manager;
pub mod online;
pub mod quest_manager;
pub mod social;

pub mod pay {
    use axum::Json;
    use serde_json::json;

    use crate::core::JSON;

    pub async fn pay_get_unconfirmed_order_id_list() -> JSON {
        Json(json!({
            "orderIdList": [],
            "playerDataDelta": {
                "deleted": {},
                "modified": {}
            }
        }))
    }

    pub async fn pay_get_all_prod_list() -> JSON {
        Json(json!({
            "productList": []
        }))
    }
}
