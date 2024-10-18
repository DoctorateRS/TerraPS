use axum::Json;
use models::{NullVec, PlayerDataDelta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecentVisitor {
    visitors: NullVec,
}

pub async fn get_recent_visitor() -> Json<RecentVisitor> {
    Json(RecentVisitor { visitors: [0; 0] })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoShareVisitorNum {
    num: u8,
}

pub async fn get_info_share_visitors_num() -> Json<InfoShareVisitorNum> {
    Json(InfoShareVisitorNum { num: 0 })
}

#[derive(Deserialize, Serialize)]
pub struct PlayerDataDeltaAssistReport {
    reports: NullVec,
    pdd: PlayerDataDelta,
}
