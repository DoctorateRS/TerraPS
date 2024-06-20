pub mod account;
pub mod background;
pub mod building;
pub mod campaignv2;
pub mod char_manager;
pub mod crisis_manager;
pub mod deep_sea;
pub mod gacha;
pub mod mail;
pub mod online;
pub mod quest_manager;
pub mod rlv2;
pub mod sandboxv2;
pub mod shop;
pub mod social;
pub mod story;
pub mod tower;

pub mod pay {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

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
