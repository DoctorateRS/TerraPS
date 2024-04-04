pub mod quest {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

    pub async fn _set_trap_squad(Json(payload): JSON) -> JSON {
        let trap_domain_id = payload["trapDomainId"].as_str().unwrap();
        let trap_squad = payload["trapSquad"].clone();

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "templateTrap": {
                        "domains": {
                            trap_domain_id: {
                                "squad": trap_squad
                            }
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }
}

pub mod bossrush {
    use axum::Json;
    use serde_json::json;

    use crate::utils::json::JSON;

    pub async fn _relic_select(Json(payload): JSON) -> JSON {
        let activity_id = payload["activityId"].as_str().unwrap();
        let relic_id = payload["relicId"].clone();

        Json(json!({
            "playerDataDelta": {
                "modified": {
                    "activity": {
                        "BOSS_RUSH": {
                            activity_id: {
                                "relic": {
                                    "select": relic_id
                                }
                            }
                        }
                    }
                },
                "deleted": {}
            }
        }))
    }
}
